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

use std::marker::PhantomData;

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::network_service_client::NetworkServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Error,
    LedgerId,
    NetworkVersionInfo,
    Query,
};

/// Get information about the versions of protobuf and hedera.
pub type NetworkVersionInfoQuery = Query<NetworkVersionInfoQueryData>;

#[derive(Default, Clone, Debug)]
pub struct NetworkVersionInfoQueryData {
    // make this not publicly constructable.
    _phantom: PhantomData<()>,
}

impl From<NetworkVersionInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: NetworkVersionInfoQueryData) -> Self {
        Self::NetworkVersionInfo(data)
    }
}

impl ToQueryProtobuf for NetworkVersionInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        services::Query {
            query: Some(services::query::Query::NetworkGetVersionInfo(
                services::NetworkGetVersionInfoQuery { header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for NetworkVersionInfoQueryData {
    type Response = NetworkVersionInfo;

    fn is_payment_required(&self) -> bool {
        false
    }

    fn validate_checksums_for_ledger_id(&self, _ledger_id: &LedgerId) -> Result<(), Error> {
        Ok(())
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        NetworkServiceClient::new(channel).get_version_info(request).await
    }
}
