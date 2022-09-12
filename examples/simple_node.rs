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

use tonic::Request;

use astarte_proto_message_hub::message_hub_client::MessageHubClient;

use crate::astarte_proto_message_hub::{Interface, Node};

pub mod astarte_proto_message_hub {
    tonic::include_proto!("astarteplatform.msghub");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = MessageHubClient::connect("http://[::1]:10000").await?;

    let interfaces = vec![
        Interface {
            name: "io.demo.ServerProperties".to_owned(),
            minor: 0,
            major: 2,
        },
        Interface {
            name: "org.astarteplatform.esp32.DeviceDatastream".to_owned(),
            minor: 0,
            major: 2,
        },
    ];

    let node_introspection = Node {
        id: "1".to_owned(),
        introspection: interfaces,
    };

    let mut stream = client
        .attach(Request::new(node_introspection))
        .await?
        .into_inner();

    while let Some(hub_msg) = stream.message().await? {
        println!("Msg = {:?}", hub_msg);
    }

    Ok(())
}
