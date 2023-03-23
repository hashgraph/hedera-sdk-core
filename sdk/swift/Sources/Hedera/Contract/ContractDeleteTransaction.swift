/*
 * ‌
 * Hedera Swift SDK
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

import GRPC
import HederaProtobufs

/// Marks a contract as deleted and transfers its remaining hBars, if any, to
/// a designated receiver.
public final class ContractDeleteTransaction: Transaction {
    /// Create a new `ContractDeleteTransaction`.
    public init(contractId: ContractId? = nil) {
        self.contractId = contractId

        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        contractId = try container.decodeIfPresent(.contractId)
        transferAccountId = try container.decodeIfPresent(.transferAccountId)
        transferContractId = try container.decodeIfPresent(.transferContractId)

        try super.init(from: decoder)
    }

    internal init(protobuf proto: Proto_TransactionBody, _ data: Proto_ContractDeleteTransactionBody) throws {
        contractId = data.hasContractID ? try .fromProtobuf(data.contractID) : nil

        switch data.obtainers {
        case .transferAccountID(let account):
            self.transferAccountId = try .fromProtobuf(account)
        case .transferContractID(let contract):
            self.transferContractId = try .fromProtobuf(contract)
        case nil:
            break

        }

        try super.init(protobuf: proto)
    }

    /// The contract to be deleted.
    public var contractId: ContractId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the contract to be deleted.
    @discardableResult
    public func contractId(_ contractId: ContractId) -> Self {
        self.contractId = contractId

        return self
    }

    /// The account ID which will receive all remaining hbars.
    public var transferAccountId: AccountId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the account ID which will receive all remaining hbars.
    @discardableResult
    public func transferAccountId(_ transferAccountId: AccountId) -> Self {
        self.transferAccountId = transferAccountId

        return self
    }

    /// The contract ID which will receive all remaining hbars.
    public var transferContractId: ContractId? {
        willSet {
            ensureNotFrozen()
        }
    }

    /// Sets the contract ID which will receive all remaining hbars.
    @discardableResult
    public func transferContractId(_ transferContractId: ContractId) -> Self {
        self.transferContractId = transferContractId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case contractId
        case transferAccountId
        case transferContractId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(contractId, forKey: .contractId)
        try container.encodeIfPresent(transferAccountId, forKey: .transferAccountId)
        try container.encodeIfPresent(transferContractId, forKey: .transferContractId)

        try super.encode(to: encoder)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try contractId?.validateChecksums(on: ledgerId)
        try transferAccountId?.validateChecksums(on: ledgerId)
        try transferContractId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }

    internal override func transactionExecute(_ channel: GRPCChannel, _ request: Proto_Transaction) async throws
        -> Proto_TransactionResponse
    {
        try await Proto_SmartContractServiceAsyncClient(channel: channel).deleteContract(request)
    }

    internal override func toTransactionDataProtobuf(_ chunkInfo: ChunkInfo) -> Proto_TransactionBody.OneOf_Data {
        _ = chunkInfo.assertSingleTransaction()

        return .contractDeleteInstance(toProtobuf())
    }
}

extension ContractDeleteTransaction: ToProtobuf {
    internal typealias Protobuf = Proto_ContractDeleteTransactionBody

    internal func toProtobuf() -> Protobuf {

        .with { proto in
            switch (transferAccountId, transferContractId) {
            case (.some, .some):
                // fixme: do what rust does
                fatalError()
            case (.some(let id), nil):
                proto.transferAccountID = id.toProtobuf()
            case (nil, .some(let id)):
                proto.transferContractID = id.toProtobuf()
            case (nil, nil):
                break
            }

            contractId?.toProtobufInto(&proto.contractID)
        }
    }
}

extension ContractDeleteTransaction: ToSchedulableTransactionData {
    internal func toSchedulableTransactionData() -> Proto_SchedulableTransactionBody.OneOf_Data {
        .contractDeleteInstance(toProtobuf())
    }
}
