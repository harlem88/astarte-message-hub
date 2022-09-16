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

use std::error::Error;

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use astarte_sdk::types::AstarteType;
use astarte_sdk::{AstarteError, Clientbound};

use crate::proto_message_hub::{AstarteMessage, Interface};

#[async_trait]
pub trait AstartePublisher: Send + Sync {
    async fn send_object<T: 'static>(
        &self,
        interface_name: &str,
        interface_path: &str,
        data: T,
    ) -> Result<(), AstarteError>
    where
        T: serde::Serialize + Send;
    //TODO add send_object_with_timestamp to this trait
    async fn send(
        &self,
        interface_name: &str,
        interface_path: &str,
        data: AstarteType,
    ) -> Result<(), AstarteError>;
}

#[async_trait]
pub trait AstarteSubscriber {
    async fn subscribe(
        introspection: Vec<Interface>,
    ) -> Result<(Receiver<AstarteMessage>), AstarteError>;
    fn unsubscribe(introspection: Vec<Interface>) -> Result<(), AstarteError>;
}
