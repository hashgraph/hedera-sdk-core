use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use hedera_proto::services;
use prost::Message;
use tonic::transport::Channel;
use tonic::{Response, Status};

use crate::execute::Execute;
use crate::transaction::protobuf::ToTransactionDataProtobuf;
use crate::{
    AccountId, Client, Error, ToProtobuf, Transaction, TransactionHash, TransactionId, TransactionResponse
};

#[async_trait]
pub trait TransactionExecute {
    fn default_max_transaction_fee() -> u64 {
        2 * 100_000_000 // 2 hbar
    }

    async fn execute(
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status>;
}

#[async_trait]
impl<D> Execute for Transaction<D>
where
    D: ToTransactionDataProtobuf,
    Self: TransactionExecute,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type Context = TransactionHash;

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.transaction_id
    }

    fn requires_transaction_id() -> bool {
        true
    }

    async fn make_request(
        &self,
        client: &Client,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        let transaction_id = transaction_id.as_ref().ok_or(Error::NoPayerAccountOrTransactionId)?;

        let transaction_body = self.to_transaction_body_protobuf(
            node_account_id,
            transaction_id,
            &client.max_transaction_fee,
        );

        let body_bytes = transaction_body.encode_to_vec();

        let mut signatures = Vec::with_capacity(self.signers.len());

        let default_signers = client.default_signers.read().await;

        for signer in default_signers.iter().chain(&self.signers) {
            // TODO: should we run the signers in parallel?
            let signature = signer.sign(&body_bytes).await.map_err(Error::signature)?;

            signatures.push(signature.to_protobuf());
        }

        let signed_transaction = services::SignedTransaction {
            body_bytes,
            sig_map: Some(services::SignatureMap { sig_pair: signatures }),
        };

        let signed_transaction_bytes = signed_transaction.encode_to_vec();

        let transaction_hash = TransactionHash::hash(&signed_transaction_bytes);

        let transaction =
            services::Transaction { signed_transaction_bytes, ..services::Transaction::default() };

        Ok((transaction, transaction_hash))
    }

    async fn execute(
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<Response<Self::GrpcResponse>, Status> {
        <Self as TransactionExecute>::execute(channel, request).await
    }

    fn make_response(
        _response: Self::GrpcResponse,
        transaction_hash: Self::Context,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(TransactionResponse {
            node_account_id,
            transaction_id: transaction_id.unwrap(),
            transaction_hash,
        })
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response.node_transaction_precheck_code)
    }
}

impl<D> Transaction<D>
where
    D: ToTransactionDataProtobuf,
    Self: TransactionExecute,
{
    #[allow(deprecated)]
    fn to_transaction_body_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
        client_max_transaction_fee: &Arc<AtomicU64>,
    ) -> services::TransactionBody {
        let data = self.data.to_transaction_data_protobuf(node_account_id, transaction_id);

        let max_transaction_fee = self.max_transaction_fee.unwrap_or_else(|| {
            // no max has been set on the *transaction*
            // check if there is a global max set on the client
            match client_max_transaction_fee.load(Ordering::Relaxed) {
                max if max > 1 => max,

                // no max has been set on the client either
                // fallback to the hard-coded default for this transaction type
                _ => Self::default_max_transaction_fee(),
            }
        });

        services::TransactionBody {
            data: Some(data),
            transaction_id: Some(transaction_id.to_protobuf()),
            transaction_valid_duration: Some(self.transaction_valid_duration.into()),
            memo: self.transaction_memo.clone(),
            node_account_id: Some(node_account_id.to_protobuf()),
            generate_record: false,
            transaction_fee: max_transaction_fee,
        }
    }
}
