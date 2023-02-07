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
use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use tonic::transport::Channel;

use crate::entity_id::AutoValidateChecksum;
use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    ContractId,
    Error,
    FromProtobuf,
    LedgerId,
    Query,
    ToProtobuf,
};

/// Get the runtime bytecode for a smart contract instance.
pub type ContractBytecodeQuery = Query<ContractBytecodeQueryData>;

#[derive(Default, Debug, Clone)]
pub struct ContractBytecodeQueryData {
    /// The contract for which information is requested.
    contract_id: Option<ContractId>,
}

impl ContractBytecodeQuery {
    /// Gets the contract for which information is requested.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data.contract_id
    }

    /// Sets the contract for which information is requested.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data.contract_id = Some(contract_id);
        self
    }
}

impl From<ContractBytecodeQueryData> for AnyQueryData {
    #[inline]
    fn from(data: ContractBytecodeQueryData) -> Self {
        Self::ContractBytecode(data)
    }
}

impl ToQueryProtobuf for ContractBytecodeQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let contract_id = self.contract_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::ContractGetBytecode(
                services::ContractGetBytecodeQuery { contract_id, header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for ContractBytecodeQueryData {
    type Response = Vec<u8>;

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksum_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        SmartContractServiceClient::new(channel).contract_get_bytecode(request).await
    }
}

impl FromProtobuf<services::response::Response> for Vec<u8> {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, ContractGetBytecodeResponse, services::response::Response);

        Ok(pb.bytecode)
    }
}
