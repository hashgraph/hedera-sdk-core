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

use crate::protobuf::ToProtobuf;
use crate::{
    AccountId,
    FromProtobuf,
    TokenId,
};

/// A token <-> account association.
#[derive(Debug, Clone)]
pub struct TokenAssociation {
    /// The token involved in the association.
    pub token_id: TokenId,

    /// The account involved in the association.
    pub account_id: AccountId,
}

impl TokenAssociation {
    /// Create a new `TokenAssociation` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl FromProtobuf<services::TokenAssociation> for TokenAssociation {
    fn from_protobuf(pb: services::TokenAssociation) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let token_id = pb_getf!(pb, token_id)?;
        let account_id = pb_getf!(pb, account_id)?;

        Ok(Self {
            token_id: TokenId::from_protobuf(token_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
        })
    }
}

impl ToProtobuf for TokenAssociation {
    type Protobuf = services::TokenAssociation;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenAssociation {
            token_id: Some(self.token_id.to_protobuf()),
            account_id: Some(self.account_id.to_protobuf()),
        }
    }
}
