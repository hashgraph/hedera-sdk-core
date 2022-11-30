/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use async_trait::async_trait;
use hedera_proto::{
    mirror,
    services,
};
use mirror::network_service_client::NetworkServiceClient;
use tonic::transport::Channel;
use tonic::Response;

use crate::mirror_query::{
    AnyMirrorQueryData,
    MirrorQuerySubscribe,
};
use crate::protobuf::FromProtobuf;
use crate::{
    FileId,
    MirrorQuery,
    NodeAddress,
    NodeAddressBook,
    ToProtobuf,
};

/// Query for an address book and return its nodes.
/// The nodes are returned in ascending order by node ID.
pub type NodeAddressBookQuery = MirrorQuery<NodeAddressBookQueryData>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct NodeAddressBookQueryData {
    /// The ID of the address book file on the network.
    /// Can either be `0.0.101` or `0.0.102`. Defaults to `0.0.102`.
    file_id: FileId,

    /// The maximum number of node addresses to receive.
    /// Defaults to _all_.
    limit: u32,
}

impl Default for NodeAddressBookQueryData {
    fn default() -> Self {
        Self { file_id: FileId::from(102), limit: 0 }
    }
}

impl NodeAddressBookQuery {
    /// Sets the ID of the address book file on the network.
    /// Can either be `0.0.101` or `0.0.102`. Defaults to `0.0.102`.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data.file_id = id.into();
        self
    }

    /// Sets the maximum number of node addresses to receive.
    /// Defaults to _all_.
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.data.limit = limit;
        self
    }
}

impl From<NodeAddressBookQueryData> for AnyMirrorQueryData {
    fn from(data: NodeAddressBookQueryData) -> Self {
        Self::NodeAddressBook(data)
    }
}

#[async_trait]
impl MirrorQuerySubscribe for NodeAddressBookQueryData {
    type GrpcStream = tonic::Streaming<services::NodeAddress>;

    type GrpcMessage = services::NodeAddress;

    type Message = NodeAddress;

    type Response = NodeAddressBook;

    fn map_response(&self, response: Vec<Self::GrpcMessage>) -> crate::Result<Self::Response> {
        NodeAddressBook::from_protobuf(services::NodeAddressBook { node_address: response })
    }

    async fn subscribe(&self, channel: Channel) -> Result<Self::GrpcStream, tonic::Status> {
        let file_id = self.file_id.to_protobuf();
        let request = mirror::AddressBookQuery { file_id: Some(file_id), limit: self.limit as i32 };

        NetworkServiceClient::new(channel).get_nodes(request).await.map(Response::into_inner)
    }

    async fn message(
        &self,
        stream: &mut Self::GrpcStream,
    ) -> Result<Option<Self::GrpcMessage>, tonic::Status> {
        stream.message().await
    }
}
