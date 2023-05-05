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

use std::collections::HashMap;
use std::ops::Not;

use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Hbar,
    LedgerId,
    NftId,
    ToProtobuf,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
pub type TransferTransaction = Transaction<TransferTransactionData>;

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct TransferTransactionData {
    transfers: Vec<Transfer>,
    token_transfers: Vec<TokenTransfer>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Transfer {
    /// The account involved in the transfer.
    account_id: AccountId,

    /// The value of the transfer.
    amount: i64,

    /// If this is an approved transfer.
    is_approval: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct TokenTransfer {
    token_id: TokenId,

    transfers: Vec<Transfer>,

    nft_transfers: Vec<NftTransfer>,

    expected_decimals: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct NftTransfer {
    sender_account_id: AccountId,
    receiver_account_id: AccountId,

    serial: u64,

    is_approval: bool,
}

impl TransferTransaction {
    fn _token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        approved: bool,
        expected_decimals: Option<u32>,
    ) -> &mut Self {
        let transfer = Transfer { account_id, amount, is_approval: approved };
        let data = self.data_mut();

        if let Some(tt) = data.token_transfers.iter_mut().find(|tt| tt.token_id == token_id) {
            tt.expected_decimals = expected_decimals;
            tt.transfers.push(transfer);
        } else {
            data.token_transfers.push(TokenTransfer {
                token_id,
                expected_decimals,
                nft_transfers: Vec::new(),
                transfers: vec![transfer],
            });
        }

        self
    }

    /// Add a non-approved token transfer to the transaction.
    pub fn token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, false, None)
    }

    /// Add an approved token transfer to the transaction.
    pub fn approved_token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, true, None)
    }

    /// Add a non-approved token transfer with decimals to the transaction.
    pub fn token_transfer_with_decimals(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        expected_decimals: u32,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, false, Some(expected_decimals))
    }

    /// Add an approved token transfer with decimals to the transaction.
    pub fn approved_token_transfer_with_decimals(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        expected_decimals: u32,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, true, Some(expected_decimals))
    }

    fn _nft_transfer(
        &mut self,
        nft_id: NftId,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
        approved: bool,
    ) -> &mut Self {
        let NftId { token_id, serial } = nft_id;
        let transfer =
            NftTransfer { serial, sender_account_id, receiver_account_id, is_approval: approved };

        let data = self.data_mut();

        if let Some(tt) = data.token_transfers.iter_mut().find(|tt| tt.token_id == token_id) {
            tt.nft_transfers.push(transfer);
        } else {
            data.token_transfers.push(TokenTransfer {
                token_id,
                expected_decimals: None,
                transfers: Vec::new(),
                nft_transfers: vec![transfer],
            });
        }

        self
    }

    /// Add an approved nft transfer to the transaction.
    pub fn approved_nft_transfer(
        &mut self,
        nft_id: impl Into<NftId>,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
    ) -> &mut Self {
        self._nft_transfer(nft_id.into(), sender_account_id, receiver_account_id, true)
    }

    /// Add a non-approved nft transfer to the transaction.
    pub fn nft_transfer(
        &mut self,
        nft_id: impl Into<NftId>,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
    ) -> &mut Self {
        self._nft_transfer(nft_id.into(), sender_account_id, receiver_account_id, false)
    }

    fn _hbar_transfer(&mut self, account_id: AccountId, amount: Hbar, approved: bool) -> &mut Self {
        self.data_mut().transfers.push(Transfer {
            account_id,
            amount: amount.to_tinybars(),
            is_approval: approved,
        });

        self
    }

    /// Add a non-approved hbar transfer to the transaction.
    pub fn hbar_transfer(&mut self, account_id: AccountId, amount: Hbar) -> &mut Self {
        self._hbar_transfer(account_id, amount, false)
    }

    /// Add an approved hbar transfer to the transaction.
    pub fn approved_hbar_transfer(&mut self, account_id: AccountId, amount: Hbar) -> &mut Self {
        self._hbar_transfer(account_id, amount, true)
    }

    /// Returns the transfers that will be executed.
    pub fn get_hbar_transfers(&self) -> HashMap<AccountId, Hbar> {
        self.data()
            .transfers
            .iter()
            .map(|it| (it.account_id, Hbar::from_tinybars(it.amount)))
            .collect()
    }
}

impl TransactionExecute for TransferTransactionData {
    // noinspection DuplicatedCode
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { CryptoServiceClient::new(channel).crypto_transfer(request).await })
    }
}

impl TransactionData for TransferTransactionData {}

impl ValidateChecksums for TransferTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        for transfer in &self.transfers {
            transfer.account_id.validate_checksums(ledger_id)?;
        }
        for token_transfer in &self.token_transfers {
            token_transfer.token_id.validate_checksums(ledger_id)?;
            for transfer in &token_transfer.transfers {
                transfer.account_id.validate_checksums(ledger_id)?;
            }
            for nft_transfer in &token_transfer.nft_transfers {
                nft_transfer.sender_account_id.validate_checksums(ledger_id)?;
                nft_transfer.receiver_account_id.validate_checksums(ledger_id)?;
            }
        }
        Ok(())
    }
}

impl FromProtobuf<services::AccountAmount> for Transfer {
    fn from_protobuf(pb: services::AccountAmount) -> crate::Result<Self> {
        Ok(Self {
            amount: pb.amount,
            account_id: AccountId::from_protobuf(pb_getf!(pb, account_id)?)?,
            is_approval: pb.is_approval,
        })
    }
}

impl ToProtobuf for Transfer {
    type Protobuf = services::AccountAmount;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountAmount {
            amount: self.amount,
            account_id: Some(self.account_id.to_protobuf()),
            is_approval: self.is_approval,
        }
    }
}

impl FromProtobuf<services::TokenTransferList> for TokenTransfer {
    fn from_protobuf(pb: services::TokenTransferList) -> crate::Result<Self> {
        Ok(Self {
            token_id: TokenId::from_protobuf(pb_getf!(pb, token)?)?,
            transfers: Vec::from_protobuf(pb.transfers)?,
            nft_transfers: Vec::from_protobuf(pb.nft_transfers)?,
            expected_decimals: pb.expected_decimals,
        })
    }
}

impl ToProtobuf for TokenTransfer {
    type Protobuf = services::TokenTransferList;

    fn to_protobuf(&self) -> Self::Protobuf {
        let transfers = self.transfers.to_protobuf();
        let nft_transfers = self.nft_transfers.to_protobuf();

        services::TokenTransferList {
            token: Some(self.token_id.to_protobuf()),
            transfers,
            nft_transfers,
            expected_decimals: self.expected_decimals,
        }
    }
}

impl FromProtobuf<services::NftTransfer> for NftTransfer {
    fn from_protobuf(pb: services::NftTransfer) -> crate::Result<Self> {
        Ok(Self {
            sender_account_id: AccountId::from_protobuf(pb_getf!(pb, sender_account_id)?)?,
            receiver_account_id: AccountId::from_protobuf(pb_getf!(pb, receiver_account_id)?)?,
            serial: pb.serial_number as u64,
            is_approval: pb.is_approval,
        })
    }
}

impl ToProtobuf for NftTransfer {
    type Protobuf = services::NftTransfer;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::NftTransfer {
            sender_account_id: Some(self.sender_account_id.to_protobuf()),
            receiver_account_id: Some(self.receiver_account_id.to_protobuf()),
            serial_number: self.serial as i64,
            is_approval: self.is_approval,
        }
    }
}

impl ToTransactionDataProtobuf for TransferTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::CryptoTransfer(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TransferTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::CryptoTransfer(self.to_protobuf())
    }
}

impl From<TransferTransactionData> for AnyTransactionData {
    fn from(transaction: TransferTransactionData) -> Self {
        Self::Transfer(transaction)
    }
}

impl FromProtobuf<services::CryptoTransferTransactionBody> for TransferTransactionData {
    fn from_protobuf(pb: services::CryptoTransferTransactionBody) -> crate::Result<Self> {
        let transfers = pb.transfers.map(|it| it.account_amounts);
        let transfers = Option::from_protobuf(transfers)?.unwrap_or_default();

        Ok(Self { transfers, token_transfers: Vec::from_protobuf(pb.token_transfers)? })
    }
}

impl ToProtobuf for TransferTransactionData {
    type Protobuf = services::CryptoTransferTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let transfers = self
            .transfers
            .is_empty()
            .not()
            .then(|| services::TransferList { account_amounts: self.transfers.to_protobuf() });

        let token_transfers = self.token_transfers.to_protobuf();

        services::CryptoTransferTransactionBody { transfers, token_transfers }
    }
}
