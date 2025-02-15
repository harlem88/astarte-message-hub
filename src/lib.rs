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
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub use crate::astarte_message_hub::AstarteMessageHub;
pub use crate::data::astarte_handler::AstarteHandler;
pub use crate::proto_message_hub::message_hub_server::MessageHubServer;

mod astarte_device_sdk_types;
mod astarte_message_hub;
pub mod config;
mod data;
mod device;
pub mod error;
#[allow(missing_docs)]
pub mod proto_message_hub;
mod types;
