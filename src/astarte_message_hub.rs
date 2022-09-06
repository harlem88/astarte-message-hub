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

use log::info;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::proto_message_hub::message_hub_server::MessageHub;
use crate::proto_message_hub::{AstarteMessage, Interface, Node};

pub struct AstarteMessageHub {
    pub nodes: Arc<Mutex<HashMap<String, AstarteNode>>>,
}

pub struct AstarteNode {
    id: String,
    introspection: Vec<Interface>,
    node_channel: Sender<Result<AstarteMessage, Status>>,
}

#[tonic::async_trait]
impl MessageHub for AstarteMessageHub {
    type AttachStream = ReceiverStream<Result<AstarteMessage, Status>>;

    async fn attach(&self, request: Request<Node>) -> Result<Response<Self::AttachStream>, Status> {
        info!("Node Attach Request => {:?}", request);
        let node = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        let astarte_node = AstarteNode {
            id: node.id.clone(),
            introspection: node.introspection,
            node_channel: tx,
        };

        let mut nodes = self.nodes.lock().await;
        nodes.insert(astarte_node.id.to_owned(), astarte_node);

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn send(&self, request: Request<AstarteMessage>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn detach(&self, request: Request<Node>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
