use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use tonic::transport::Channel;

use crate::file::FileInfo; //ask about
use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{FileId, Query, ToProtobuf}; //ask about

/// Get all the information about a file.
pub type FileInfoQuery = Query<FileInfoQueryData>; //ask about

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileInfoQueryData {
    file_id: Option<FileId>, //ask about (used to be AccountAddress)
}

impl From<FileInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: FileInfoQueryData) -> Self {
        Self::FileInfo(data) //ask about (used to be AccountInfo)
    }
}

impl FileInfoQuery {
    /// Sets the file ID for which information is requested.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.data.file_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for FileInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let file_id = self.file_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::FileGetInfo(services::FileGetInfoQuery {
                file_id,
                header: Some(header),
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for FileInfoQueryData {
    type Response = FileInfo;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        FileServiceClient::new(channel).get_file_info(request).await
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::query::AnyQueryData;
    use crate::{AccountId, FileId, FileInfoQuery, AnyQuery};

    // language=JSON
    const FILE_INFO: &str = r#"{
  "$type": "fileInfo",
  "fileId": "0.0.1001",
  "payment": {
    "amount": 50,
    "transactionMemo": "query payment",
    "payerAccountId": "0.0.6189"
  }
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut query = FileInfoQuery::new();
        query
            .file_id(FileId::from(1001))
            .payer_account_id(AccountId::from(6189))
            .payment_amount(50)
            .payment_transaction_memo("query payment");

        let s = serde_json::to_string_pretty(&query)?;
        assert_eq!(s, FILE_INFO);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let query: AnyQuery = serde_json::from_str(FILE_INFO)?;

        let data = assert_matches!(query.data, AnyQueryData::FileInfo(query) => query);

        assert_eq!(data.file_id, Some(FileId{shard:0, realm:0, num:1001}));
        assert_eq!(query.payment.body.data.amount, Some(50));
        assert_eq!(query.payment.body.transaction_memo, "query payment");
        assert_eq!(query.payment.body.payer_account_id, Some(AccountId::from(6189)));

        Ok(())
    }
}

