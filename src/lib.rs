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

use serde::Deserialize;

pub use crate::astarte_message_hub::AstarteMessageHub;
pub use crate::data::astarte::{astarte_map_options, Astarte};
pub use crate::proto_message_hub::message_hub_server::MessageHubServer;

mod astarte_message_hub;
mod astarte_sdk_types;
mod data;
pub mod error;
mod types;

pub mod proto_message_hub {
    tonic::include_proto!("astarteplatform.msghub");
}

#[derive(Debug, Deserialize, Clone)]
pub struct AstarteMessageHubOptions {
    pub realm: String,
    pub device_id: Option<String>,
    pub credentials_secret: Option<String>,
    pub pairing_url: String,
    pub pairing_token: Option<String>,
    pub interfaces_directory: String,
    pub store_directory: String,
    pub astarte_ignore_ssl: Option<bool>,
}
