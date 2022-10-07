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
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{
    serde_as,
    skip_serializing_none,
};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    TokenId,
    Transaction,
    TransactionId,
};

/// Dissociates the provided account with the provided tokens. Must be signed by the provided
/// Account's key.
///
/// On success, associations between the provided account and tokens are removed.
///
/// - If the provided account is not found, the transaction will resolve to `INVALID_ACCOUNT_ID`.
/// - If the provided account has been deleted, the transaction will resolve to `ACCOUNT_DELETED`.
/// - If any of the provided tokens is not found, the transaction will resolve to `INVALID_TOKEN_REF`.
/// - If any of the provided tokens has been deleted, the transaction will resolve to `TOKEN_WAS_DELETED`.
/// - If an association between the provided account and any of the tokens does not exist, the
/// transaction will resolve to `TOKEN_NOT_ASSOCIATED_TO_ACCOUNT`.
/// - If a token has not been deleted and has not expired, and the user has a nonzero balance, the
/// transaction will resolve to `TRANSACTION_REQUIRES_ZERO_TOKEN_BALANCES`.
/// - If a <b>fungible token</b> has expired, the user can disassociate even if their token balance is
/// not zero.
/// - If a <b>non fungible token</b> has expired, the user can <b>not</b> disassociate if their token
/// balance is not zero. The transaction will resolve to `TRANSACTION_REQUIRED_ZERO_TOKEN_BALANCES`.
pub type TokenDissociateTransaction = Transaction<TokenDissociateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDissociateTransactionData {
    /// The account to be dissociated with the provided tokens.
    account_id: Option<AccountId>,

    /// The tokens to be dissociated with the provided account.
    token_ids: Vec<TokenId>,
}

impl TokenDissociateTransaction {
    /// Sets the account to be dissociated with the provided tokens.
    pub fn account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.body.data.account_id = Some(account_id);
        self
    }

    /// Sets the tokens to be dissociated with the provided account.
    pub fn token_ids(&mut self, token_ids: impl IntoIterator<Item = TokenId>) -> &mut Self {
        self.body.data.token_ids = token_ids.into_iter().collect();
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenDissociateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).dissociate_tokens(request).await
    }
}

impl ToTransactionDataProtobuf for TokenDissociateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let account = self.account_id.as_ref().map(AccountId::to_protobuf);
        let tokens = self.token_ids.iter().map(TokenId::to_protobuf).collect();

        services::transaction_body::Data::TokenDissociate(
            services::TokenDissociateTransactionBody { account, tokens },
        )
    }
}

impl From<TokenDissociateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenDissociateTransactionData) -> Self {
        Self::TokenDissociate(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{
        AnyTransaction,
        AnyTransactionData,
    };
    use crate::{
        AccountId,
        TokenDissociateTransaction,
        TokenId,
    };

    // language=JSON
    const TOKEN_DISSOCIATE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenDissociate",
  "accountId": "0.0.1001",
  "tokenIds": [
    "0.0.1002",
    "0.0.1003"
  ]
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = TokenDissociateTransaction::new();

        transaction
            .account_id(AccountId::from(1001))
            .token_ids([TokenId::from(1002), TokenId::from(1003)]);

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, TOKEN_DISSOCIATE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(TOKEN_DISSOCIATE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::TokenDissociate(transaction) => transaction);

        assert_eq!(data.token_ids, [TokenId::from(1002), TokenId::from(1003)]);
        assert_eq!(data.account_id, Some(AccountId::from(1001)));

        Ok(())
    }
}
