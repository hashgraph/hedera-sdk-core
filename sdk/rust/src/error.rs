use std::error::Error as StdError;
use std::result::Result as StdResult;

use hedera_proto::services::ResponseCodeEnum;

use crate::{AccountId, TransactionId};

pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to complete request within the maximum time allowed; most recent attempt failed with: {0}")]
    TimedOut(Box<Error>),

    #[error("grpc: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("failed to parse a protobuf response: {0}")]
    FromProtobuf(BoxStdError),

    /// Signals that a query or transaction has failed the pre-check.
    // FIXME: Use hedera::Status (once available)
    // TODO: Add transaction_id: Option<TransactionId>
    #[error("transaction `{}` failed pre-check with status `{status:?}`", .transaction_id.as_ref().map(|id| id.to_string()).as_deref().unwrap_or("_"))]
    PreCheckStatus { status: ResponseCodeEnum, transaction_id: Option<TransactionId> },

    #[error("failed to parse a key: {0}")]
    KeyParse(BoxStdError),

    #[error("client must be configured with a payer account or requests must be given an explicit transaction id")]
    NoPayerAccountOrTransactionId,

    #[error("exceeded maximum attempts for request; most recent attempt failed with: {0}")]
    MaxAttemptsExceededException(Box<Error>),

    #[error("node account `{0}` was not found in the configured network")]
    NodeAccountUnknown(AccountId),

    #[error("received unrecognized status code: {0}, try updating your SDK")]
    ResponseStatusUnrecognized(i32),

    #[error("failed to sign request: {0}")]
    Signature(BoxStdError),
}

impl Error {
    pub(crate) fn from_protobuf<E: Into<BoxStdError>>(error: E) -> Self {
        Self::FromProtobuf(error.into())
    }

    pub(crate) fn key_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyParse(error.into())
    }

    pub(crate) fn signature<E: Into<BoxStdError>>(error: E) -> Self {
        Self::Signature(error.into())
    }

    pub(crate) fn pre_check(
        status: ResponseCodeEnum,
        transaction_id: Option<TransactionId>,
    ) -> Self {
        Self::PreCheckStatus { status, transaction_id }
    }
}
