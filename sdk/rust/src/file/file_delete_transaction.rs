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

use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use tonic::transport::Channel;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    BoxGrpcFuture,
    Error,
    FileId,
    LedgerId,
    Transaction,
    ValidateChecksums,
};

/// Delete the given file.
///
/// After deletion, it will be marked as deleted and will have no contents.
/// Information about it will continue to exist until it expires.
///
pub type FileDeleteTransaction = Transaction<FileDeleteTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct FileDeleteTransactionData {
    /// The file to delete. It will be marked as deleted until it expires.
    /// Then it will disappear.
    file_id: Option<FileId>,
}

impl FileDeleteTransaction {
    /// Returns the ID of the file to be deleted.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the ID of the file to be deleted.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data_mut().file_id = Some(id.into());
        self
    }
}

impl TransactionData for FileDeleteTransactionData {}

impl TransactionExecute for FileDeleteTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { FileServiceClient::new(channel).delete_file(request).await })
    }
}

impl ValidateChecksums for FileDeleteTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for FileDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::FileDelete(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for FileDeleteTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::FileDelete(self.to_protobuf())
    }
}

impl From<FileDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: FileDeleteTransactionData) -> Self {
        Self::FileDelete(transaction)
    }
}

impl FromProtobuf<services::FileDeleteTransactionBody> for FileDeleteTransactionData {
    fn from_protobuf(pb: services::FileDeleteTransactionBody) -> crate::Result<Self> {
        Ok(Self { file_id: Option::from_protobuf(pb.file_id)? })
    }
}

impl ToProtobuf for FileDeleteTransactionData {
    type Protobuf = services::FileDeleteTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::FileDeleteTransactionBody { file_id: self.file_id.to_protobuf() }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use crate::{
            FileDeleteTransaction,
            FileId,
        };

        // language=JSON
        const FILE_DELETE_TRANSACTION_JSON: &str = r#"{
  "$type": "fileDelete",
  "fileId": "0.0.1001"
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = FileDeleteTransaction::new();

            transaction.file_id(FileId::from(1001));

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, FILE_DELETE_TRANSACTION_JSON);

            Ok(())
        }
    }
}
