/*
 * This file is part of Astarte.
 *
 * Copyright 2022 SECO Mind Srl
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use astarte_sdk::builder::AstarteOptions;
use astarte_sdk::types::AstarteType;
use astarte_sdk::{Aggregation, AstarteError, AstarteSdk};
use async_trait::async_trait;
use serde::Serialize;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use tonic::Status;

use crate::astarte_message_hub::AstarteNode;
use crate::data::{AstartePublisher, AstarteSubscriber};
use crate::proto_message_hub::astarte_message::Payload;
use crate::proto_message_hub::{AstarteDataType, AstarteMessage};
use crate::{proto_message_hub, AstarteMessageHubOptions};

pub struct Astarte {
    pub device_sdk: AstarteSdk,
    subscribers: Arc<Mutex<HashMap<String, Subscriber>>>,
}

struct Subscriber {
    introspection: Vec<proto_message_hub::Interface>,
    sender: Sender<Result<AstarteMessage, Status>>,
}

#[async_trait]
impl AstartePublisher for Astarte {
    async fn send_object<T>(
        &self,
        interface_name: &str,
        interface_path: &str,
        data: T,
    ) -> Result<(), AstarteError>
    where
        T: Serialize + Send,
    {
        self.device_sdk
            .send_object(interface_name, interface_path, data)
            .await
    }

    async fn send(
        &self,
        interface_name: &str,
        interface_path: &str,
        data: AstarteType,
    ) -> Result<(), AstarteError> {
        self.device_sdk
            .send(interface_name, interface_path, data)
            .await
    }
}

#[async_trait]
impl AstarteSubscriber for Astarte {
    async fn subscribe(
        &self,
        astarte_node: &AstarteNode,
    ) -> Result<Receiver<Result<AstarteMessage, Status>>, AstarteError> {
        let (tx, rx) = mpsc::channel(2);
        //TODO check introspection is already present or is present with different major/minor
        self.subscribers.lock().await.insert(
            astarte_node.id.clone(),
            Subscriber {
                introspection: astarte_node.introspection.clone(),
                sender: tx,
            },
        );

        //TODO resend introspection to Astarte
        Ok(rx)
    }

    async fn unsubscribe(&self, astarte_node: AstarteNode) -> Result<(), AstarteError> {
        //TODO resend introspection to Astarte
        self.subscribers.lock().await.remove(&astarte_node.id);

        Ok(())
    }
}

pub async fn astarte_map_options(
    opts: &AstarteMessageHubOptions,
) -> Result<AstarteOptions, AstarteError> {
    let device_id: String = opts.device_id.clone().unwrap();
    let credentials_secret: String = opts.credentials_secret.clone().unwrap();

    let mut sdk_options = AstarteOptions::new(
        &opts.realm,
        &device_id,
        &credentials_secret,
        &opts.pairing_url,
    );

    if Some(true) == opts.astarte_ignore_ssl {
        sdk_options.ignore_ssl_errors();
    }

    Ok(sdk_options
        .interface_directory(&opts.interfaces_directory)?
        .build())
}

impl Astarte {
    pub async fn new(sdk_options: AstarteOptions) -> Result<Astarte, AstarteError> {
        let device = AstarteSdk::new(&sdk_options).await?;
        Ok(Astarte {
            device_sdk: device,
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn run(&mut self) {
        use crate::proto_message_hub::AstarteUnsetType;

        match self.device_sdk.poll().await {
            Ok(clientbound) => {
                println!("incoming: {:?}", clientbound);

                let payload = if let Aggregation::Individual(individual_payload) = clientbound.data
                {
                    if let AstarteType::Unset = individual_payload {
                        Payload::AstarteUnset(AstarteUnsetType {})
                    } else {
                        let astarte_type: AstarteDataType = individual_payload.try_into().unwrap();
                        Payload::AstarteData(astarte_type)
                    }
                } else {
                    //TODO convert to object
                    return;
                };

                let astarte_message = AstarteMessage {
                    interface: Some(proto_message_hub::Interface {
                        name: clientbound.interface.clone(),
                        major: 0,
                        minor: 0,
                    }),
                    path: clientbound.path.clone(),
                    payload: Some(payload),
                    timestamp: Some(SystemTime::now().into()),
                };

                let subs = self.subscribers.lock().await;

                let subs = subs
                    .iter()
                    .filter(|(_, subscriber)| {
                        subscriber
                            .introspection
                            .iter()
                            .map(|iface| iface.name.clone())
                            .collect::<String>()
                            .contains(&clientbound.interface)
                    })
                    .collect::<HashMap<&String, &Subscriber>>();
                for (_, subscriber) in subs {
                    let _ = subscriber.sender.send(Ok(astarte_message.clone())).await;
                }
            }
            _ => {}
        }
    }
}
