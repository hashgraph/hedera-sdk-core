use async_trait::async_trait;
use hedera_proto::services;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, skip_serializing_none, DurationSeconds};
use time::Duration;
use tonic::transport::Channel;
use tonic::{Response, Status};

use crate::account::{
    AccountCreateTransactionData, AccountDeleteTransactionData, AccountUpdateTransactionData
};
use crate::contract::{
    ContractCreateTransactionData, ContractDeleteTransactionData, ContractExecuteTransactionData, ContractUpdateTransactionData
};
use crate::file::{
    FileAppendTransactionData, FileCreateTransactionData, FileDeleteTransactionData, FileUpdateTransactionData
};
use crate::token::{
    TokenAssociateTransactionData, TokenBurnTransactionData, TokenCreateTransactionData, TokenDeleteTransactionData, TokenDissociateTransactionData, TokenFeeScheduleUpdateTransactionData, TokenFreezeTransactionData, TokenGrantKycTransactionData, TokenMintTransactionData, TokenPauseTransactionData, TokenRevokeKycTransactionData, TokenUnfreezeTransactionData, TokenUnpauseTransactionData, TokenWipeTransactionData
};
use crate::topic::{
    TopicCreateTransactionData, TopicDeleteTransactionData, TopicMessageSubmitTransactionData, TopicUpdateTransactionData
};
use crate::transaction::{ToTransactionDataProtobuf, TransactionBody, TransactionExecute};
use crate::transfer_transaction::TransferTransactionData;
use crate::{AccountId, Transaction, TransactionId};

/// Any possible transaction that may be executed on the Hedera network.
pub type AnyTransaction = Transaction<AnyTransactionData>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum AnyTransactionData {
    AccountCreate(AccountCreateTransactionData),
    AccountUpdate(AccountUpdateTransactionData),
    AccountDelete(AccountDeleteTransactionData),
    ContractCreate(ContractCreateTransactionData),
    ContractUpdate(ContractUpdateTransactionData),
    ContractDelete(ContractDeleteTransactionData),
    ContractExecute(ContractExecuteTransactionData),
    Transfer(TransferTransactionData),
    TopicCreate(TopicCreateTransactionData),
    TopicUpdate(TopicUpdateTransactionData),
    TopicDelete(TopicDeleteTransactionData),
    TopicMessageSubmit(TopicMessageSubmitTransactionData),
    FileAppend(FileAppendTransactionData),
    FileCreate(FileCreateTransactionData),
    FileUpdate(FileUpdateTransactionData),
    FileDelete(FileDeleteTransactionData),
    TokenAssociate(TokenAssociateTransactionData),
    TokenBurn(TokenBurnTransactionData),
    TokenCreate(TokenCreateTransactionData),
    TokenDelete(TokenDeleteTransactionData),
    TokenDissociate(TokenDissociateTransactionData),
    TokenFeeScheduleUpdate(TokenFeeScheduleUpdateTransactionData),
    TokenFreeze(TokenFreezeTransactionData),
    TokenGrantKyc(TokenGrantKycTransactionData),
    TokenMint(TokenMintTransactionData),
    TokenPause(TokenPauseTransactionData),
    TokenRevokeKyc(TokenRevokeKycTransactionData),
    TokenUnfreeze(TokenUnfreezeTransactionData),
    TokenUnpause(TokenUnpauseTransactionData),
    TokenWipe(TokenWipeTransactionData),
}

impl ToTransactionDataProtobuf for AnyTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        match self {
            Self::Transfer(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractExecute(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileAppend(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenAssociate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenBurn(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenDissociate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenFeeScheduleUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenFreeze(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenGrantKyc(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenMint(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenPause(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenRevokeKyc(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenUnfreeze(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenUnpause(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenWipe(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicMessageSubmit(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }
        }
    }
}

#[async_trait]
impl TransactionExecute for AnyTransactionData {
    fn default_max_transaction_fee(&self) -> u64 {
        match self {
            Self::Transfer(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractExecute(transaction) => transaction.default_max_transaction_fee(),
            Self::FileAppend(transaction) => transaction.default_max_transaction_fee(),
            Self::FileCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::FileUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::FileDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenAssociate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenBurn(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenDissociate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenFeeScheduleUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenFreeze(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenGrantKyc(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenMint(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenPause(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenRevokeKyc(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenUnfreeze(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenUnpause(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenWipe(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicMessageSubmit(transaction) => transaction.default_max_transaction_fee(),
        }
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        match self {
            Self::Transfer(transaction) => transaction.execute(channel, request).await,
            Self::AccountCreate(transaction) => transaction.execute(channel, request).await,
            Self::AccountUpdate(transaction) => transaction.execute(channel, request).await,
            Self::AccountDelete(transaction) => transaction.execute(channel, request).await,
            Self::ContractCreate(transaction) => transaction.execute(channel, request).await,
            Self::ContractUpdate(transaction) => transaction.execute(channel, request).await,
            Self::ContractDelete(transaction) => transaction.execute(channel, request).await,
            Self::ContractExecute(transaction) => transaction.execute(channel, request).await,
            Self::FileAppend(transaction) => transaction.execute(channel, request).await,
            Self::FileCreate(transaction) => transaction.execute(channel, request).await,
            Self::FileUpdate(transaction) => transaction.execute(channel, request).await,
            Self::FileDelete(transaction) => transaction.execute(channel, request).await,
            Self::TokenAssociate(transaction) => transaction.execute(channel, request).await,
            Self::TokenBurn(transaction) => transaction.execute(channel, request).await,
            Self::TokenCreate(transaction) => transaction.execute(channel, request).await,
            Self::TokenDelete(transaction) => transaction.execute(channel, request).await,
            Self::TokenDissociate(transaction) => transaction.execute(channel, request).await,
            Self::TokenFeeScheduleUpdate(transaction) => {
                transaction.execute(channel, request).await
            }
            Self::TokenFreeze(transaction) => transaction.execute(channel, request).await,
            Self::TokenGrantKyc(transaction) => transaction.execute(channel, request).await,
            Self::TokenMint(transaction) => transaction.execute(channel, request).await,
            Self::TokenPause(transaction) => transaction.execute(channel, request).await,
            Self::TokenRevokeKyc(transaction) => transaction.execute(channel, request).await,
            Self::TokenUnfreeze(transaction) => transaction.execute(channel, request).await,
            Self::TokenUnpause(transaction) => transaction.execute(channel, request).await,
            Self::TokenWipe(transaction) => transaction.execute(channel, request).await,
            Self::TopicCreate(transaction) => transaction.execute(channel, request).await,
            Self::TopicUpdate(transaction) => transaction.execute(channel, request).await,
            Self::TopicDelete(transaction) => transaction.execute(channel, request).await,
            Self::TopicMessageSubmit(transaction) => transaction.execute(channel, request).await,
        }
    }
}

// NOTE: as we cannot derive Deserialize on Query<T> directly as `T` is not Deserialize,
//  we create a proxy type that has the same layout but is only for AnyQueryData and does
//  derive(Deserialize).

#[serde_as]
#[skip_serializing_none]
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AnyTransactionBody<D> {
    #[serde(flatten)]
    data: D,

    #[serde(default)]
    node_account_ids: Option<Vec<AccountId>>,

    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    #[serde(default)]
    transaction_valid_duration: Option<Duration>,

    #[serde(default)]
    max_transaction_fee: Option<u64>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    transaction_memo: String,

    #[serde(default)]
    payer_account_id: Option<AccountId>,

    #[serde(default)]
    transaction_id: Option<TransactionId>,
}

impl<D> From<AnyTransactionBody<D>> for Transaction<D>
where
    D: TransactionExecute,
{
    fn from(body: AnyTransactionBody<D>) -> Self {
        Self { body: body.into(), signers: Vec::new() }
    }
}

impl<D> From<TransactionBody<D>> for AnyTransactionBody<D>
where
    D: TransactionExecute,
{
    fn from(body: TransactionBody<D>) -> Self {
        Self {
            data: body.data,
            node_account_ids: body.node_account_ids,
            transaction_valid_duration: body.transaction_valid_duration,
            max_transaction_fee: body.max_transaction_fee,
            transaction_memo: body.transaction_memo,
            payer_account_id: body.payer_account_id,
            transaction_id: body.transaction_id,
        }
    }
}

impl<D> From<AnyTransactionBody<D>> for TransactionBody<D>
where
    D: TransactionExecute,
{
    fn from(body: AnyTransactionBody<D>) -> Self {
        Self {
            data: body.data,
            node_account_ids: body.node_account_ids,
            transaction_valid_duration: body.transaction_valid_duration,
            max_transaction_fee: body.max_transaction_fee,
            transaction_memo: body.transaction_memo,
            payer_account_id: body.payer_account_id,
            transaction_id: body.transaction_id,
        }
    }
}

impl<'de> Deserialize<'de> for AnyTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <AnyTransactionBody<AnyTransactionData> as Deserialize>::deserialize(deserializer)
            .map(AnyTransactionBody::into)
    }
}
