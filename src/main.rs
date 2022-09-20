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

use std::net::SocketAddr;

use clap::Parser;
use log::info;
use tonic::transport::Server;

use astarte_message_hub::MessageHubServer;
use astarte_message_hub::{astarte_map_options, Astarte, AstarteMessageHub};
use config::read_options;

mod config;

#[derive(Debug, Parser)]
struct Cli {
    /// Override configuration file path
    #[clap(short, long)]
    configuration_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr: SocketAddr = "[::1]:10000".parse().unwrap();

    let Cli {
        configuration_file: config_file_path,
    } = Parser::parse();

    let options = read_options(config_file_path)?;

    let astarte_options = astarte_map_options(&options).await?;
    let astarte = Astarte::new(astarte_options).await?;
    let astarte_message_hub = AstarteMessageHub::new(astarte);

    let astarte_message_server = MessageHubServer::new(astarte_message_hub);
    Server::builder()
        .add_service(astarte_message_server)
        .serve(addr)
        .await?;
    info!("Astarte Message Hub listening on: {}", addr);

    Ok(())
}
