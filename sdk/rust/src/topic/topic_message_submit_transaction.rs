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

use std::cmp;
use std::num::NonZeroUsize;

use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use tonic::transport::Channel;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkData,
    ChunkInfo,
    ChunkedTransactionData,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
    TransactionExecuteChunked,
};
use crate::{
    BoxGrpcFuture,
    Error,
    LedgerId,
    TopicId,
    Transaction,
    ValidateChecksums,
};

/// Submit a message for consensus.
///
/// Valid and authorized messages on valid topics will be ordered by the consensus service, gossipped to the
/// mirror net, and published (in order) to all subscribers (from the mirror net) on this topic.
///
/// The `submit_key` (if any) must sign this transaction.
///
/// On success, the resulting `TransactionReceipt` contains the topic's updated `topic_sequence_number` and
/// `topic_running_hash`.
///
pub type TopicMessageSubmitTransaction = Transaction<TopicMessageSubmitTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct TopicMessageSubmitTransactionData {
    /// The topic ID to submit this message to.
    topic_id: Option<TopicId>,

    #[cfg_attr(feature = "ffi", serde(flatten))]
    chunk_data: ChunkData,
}

impl TopicMessageSubmitTransaction {
    /// Returns the ID of the topic this message will be submitted to.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data().topic_id
    }

    /// Sets the topic ID to submit this message to.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data_mut().topic_id = Some(id.into());
        self
    }

    /// Returns the message to be submitted.
    pub fn get_message(&self) -> Option<&[u8]> {
        Some(self.data().chunk_data.data.as_slice())
    }

    /// Sets the message to be submitted.
    pub fn message(&mut self, bytes: impl Into<Vec<u8>>) -> &mut Self {
        self.data_mut().chunk_data_mut().data = bytes.into();
        self
    }
}

impl TransactionData for TopicMessageSubmitTransactionData {
    fn maybe_chunk_data(&self) -> Option<&ChunkData> {
        Some(self.chunk_data())
    }

    fn wait_for_receipt(&self) -> bool {
        false
    }
}

impl ChunkedTransactionData for TopicMessageSubmitTransactionData {
    fn chunk_data(&self) -> &ChunkData {
        &self.chunk_data
    }

    fn chunk_data_mut(&mut self) -> &mut ChunkData {
        &mut self.chunk_data
    }
}

impl TransactionExecute for TopicMessageSubmitTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ConsensusServiceClient::new(channel).submit_message(request).await })
    }
}

impl TransactionExecuteChunked for TopicMessageSubmitTransactionData {}

impl ValidateChecksums for TopicMessageSubmitTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TopicMessageSubmitTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        services::transaction_body::Data::ConsensusSubmitMessage(
            services::ConsensusSubmitMessageTransactionBody {
                topic_id: self.topic_id.to_protobuf(),
                message: self.chunk_data.message_chunk(chunk_info).to_vec(),
                chunk_info: (chunk_info.total > 1).then(|| services::ConsensusMessageChunkInfo {
                    initial_transaction_id: Some(chunk_info.initial_transaction_id.to_protobuf()),
                    number: (chunk_info.current + 1) as i32,
                    total: chunk_info.total as i32,
                }),
            },
        )
    }
}

impl ToSchedulableTransactionDataProtobuf for TopicMessageSubmitTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        assert!(
            self.chunk_data.used_chunks() == 1,
            "Cannot schedule a `TopicMessageSubmitTransaction` with multiple chunks"
        );

        let data = services::ConsensusSubmitMessageTransactionBody {
            topic_id: self.topic_id.to_protobuf(),
            message: self.chunk_data.data.clone(),
            chunk_info: None,
        };

        services::schedulable_transaction_body::Data::ConsensusSubmitMessage(data)
    }
}

impl From<TopicMessageSubmitTransactionData> for AnyTransactionData {
    fn from(transaction: TopicMessageSubmitTransactionData) -> Self {
        Self::TopicMessageSubmit(transaction)
    }
}

impl FromProtobuf<services::ConsensusSubmitMessageTransactionBody>
    for TopicMessageSubmitTransactionData
{
    fn from_protobuf(pb: services::ConsensusSubmitMessageTransactionBody) -> crate::Result<Self> {
        Self::from_protobuf(Vec::from([pb]))
    }
}

impl FromProtobuf<Vec<services::ConsensusSubmitMessageTransactionBody>>
    for TopicMessageSubmitTransactionData
{
    fn from_protobuf(
        pb: Vec<services::ConsensusSubmitMessageTransactionBody>,
    ) -> crate::Result<Self> {
        let total_chunks = pb.len();

        let mut iter = pb.into_iter();
        let pb_first = iter.next().expect("Empty transaction (should've been handled earlier)");

        let topic_id = Option::from_protobuf(pb_first.topic_id)?;

        let mut largest_chunk_size = pb_first.message.len();
        let mut message = pb_first.message;

        // note: no other SDK checks for correctness here... so let's not do it here either?

        for item in iter {
            largest_chunk_size = cmp::max(largest_chunk_size, item.message.len());
            message.extend_from_slice(&item.message);
        }

        Ok(Self {
            topic_id,
            chunk_data: ChunkData {
                max_chunks: total_chunks,
                chunk_size: NonZeroUsize::new(largest_chunk_size)
                    .unwrap_or_else(|| NonZeroUsize::new(1).unwrap()),
                data: message,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use crate::{
            TopicId,
            TopicMessageSubmitTransaction,
        };

        // language=JSON
        const TOPIC_MESSAGE_SUBMIT_TRANSACTION_JSON: &str = r#"{
  "$type": "topicMessageSubmit",
  "topicId": "0.0.1001",
  "maxChunks": 1,
  "data": "TWVzc2FnZQ=="
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TopicMessageSubmitTransaction::new();

            transaction.topic_id(TopicId::from(1001)).message("Message").max_chunks(1);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOPIC_MESSAGE_SUBMIT_TRANSACTION_JSON);

            Ok(())
        }
    }
}
