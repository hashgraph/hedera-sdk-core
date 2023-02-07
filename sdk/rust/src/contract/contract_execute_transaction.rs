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
use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    ContractId,
    Error,
    Hbar,
    LedgerId,
    ToProtobuf,
    Transaction,
};

/// Call a function of the given smart contract instance, giving it
/// parameters as its inputs.
///
/// It can use the given amount of gas, and any unspent gas will
/// be refunded to the paying account.
///
/// If this function stores information, it is charged gas to store it.
/// There is a fee in hbars to maintain that storage until the expiration time,
/// and that fee is added as part of the transaction fee.
///
pub type ContractExecuteTransaction = Transaction<ContractExecuteTransactionData>;

#[derive(Default, Debug, Clone)]
pub struct ContractExecuteTransactionData {
    /// The contract instance to call.
    contract_id: Option<ContractId>,

    /// The maximum amount of gas to use for the call.
    gas: u64,

    /// The number of hbars sent with this function call.
    payable_amount: Hbar,

    /// The function parameters as their raw bytes.
    function_parameters: Vec<u8>,
}

impl ContractExecuteTransaction {
    /// Returns the contract instance to call.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data().contract_id
    }

    /// Sets the contract instance to call.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data_mut().contract_id = Some(contract_id);
        self
    }

    /// Returns the maximum amount of gas to use for the call.
    #[must_use]
    pub fn get_gas(&self) -> u64 {
        self.data().gas
    }

    /// Sets the maximum amount of gas to use for the call.
    pub fn gas(&mut self, gas: u64) -> &mut Self {
        self.data_mut().gas = gas;
        self
    }

    /// Returns the number of hbars to be sent with this function call.
    #[must_use]
    pub fn get_payable_amount(&self) -> Hbar {
        self.data().payable_amount
    }

    /// Sets the number of hbars to be sent with this function call.
    pub fn payable_amount(&mut self, amount: Hbar) -> &mut Self {
        self.data_mut().payable_amount = amount;
        self
    }

    /// Returns the function parameters as their raw bytes.
    #[must_use]
    pub fn get_function_parameters(&self) -> &[u8] {
        &self.data().function_parameters
    }

    /// Sets the function parameters as their raw bytes.
    pub fn function_parameters(&mut self, data: Vec<u8>) -> &mut Self {
        self.data_mut().function_parameters = data;
        self
    }
}

#[async_trait]
impl TransactionExecute for ContractExecuteTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksum_for_ledger_id(ledger_id)?;
        Ok(())
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        SmartContractServiceClient::new(channel).contract_call_method(request).await
    }
}

impl ToTransactionDataProtobuf for ContractExecuteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.to_protobuf();

        services::transaction_body::Data::ContractCall(
            #[allow(deprecated)]
            services::ContractCallTransactionBody {
                gas: self.gas as i64,
                amount: self.payable_amount.to_tinybars(),
                contract_id,
                function_parameters: self.function_parameters.clone(),
            },
        )
    }
}

impl From<ContractExecuteTransactionData> for AnyTransactionData {
    fn from(transaction: ContractExecuteTransactionData) -> Self {
        Self::ContractExecute(transaction)
    }
}

impl FromProtobuf<services::ContractCallTransactionBody> for ContractExecuteTransactionData {
    fn from_protobuf(pb: services::ContractCallTransactionBody) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            contract_id: Option::from_protobuf(pb.contract_id)?,
            gas: pb.gas as u64,
            payable_amount: Hbar::from_tinybars(pb.amount),
            function_parameters: pb.function_parameters,
        })
    }
}
