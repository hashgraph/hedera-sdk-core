// DO NOT EDIT.
// swift-format-ignore-file
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: transaction_record.proto
//
// For information on using the generated types, please see the documentation:
//   https://github.com/apple/swift-protobuf/

import Foundation
import SwiftProtobuf

// If the compiler emits an error on this type, it is because this file
// was generated by a version of the `protoc` Swift plug-in that is
// incompatible with the version of SwiftProtobuf to which you are linking.
// Please ensure that you are building against the same version of the API
// that was used to generate this file.
fileprivate struct _GeneratedWithProtocGenSwiftVersion: SwiftProtobuf.ProtobufAPIVersionCheck {
  struct _2: SwiftProtobuf.ProtobufAPIVersion_2 {}
  typealias Version = _2
}

///*
/// Response when the client sends the node TransactionGetRecordResponse
public struct Proto_TransactionRecord {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  ///*
  /// The status (reach consensus, or failed, or is unknown) and the ID of any new
  /// account/file/instance created.
  public var receipt: Proto_TransactionReceipt {
    get {return _storage._receipt ?? Proto_TransactionReceipt()}
    set {_uniqueStorage()._receipt = newValue}
  }
  /// Returns true if `receipt` has been explicitly set.
  public var hasReceipt: Bool {return _storage._receipt != nil}
  /// Clears the value of `receipt`. Subsequent reads from it will return its default value.
  public mutating func clearReceipt() {_uniqueStorage()._receipt = nil}

  ///*
  /// The hash of the Transaction that executed (not the hash of any Transaction that failed for
  /// having a duplicate TransactionID)
  public var transactionHash: Data {
    get {return _storage._transactionHash}
    set {_uniqueStorage()._transactionHash = newValue}
  }

  ///*
  /// The consensus timestamp (or null if didn't reach consensus yet)
  public var consensusTimestamp: Proto_Timestamp {
    get {return _storage._consensusTimestamp ?? Proto_Timestamp()}
    set {_uniqueStorage()._consensusTimestamp = newValue}
  }
  /// Returns true if `consensusTimestamp` has been explicitly set.
  public var hasConsensusTimestamp: Bool {return _storage._consensusTimestamp != nil}
  /// Clears the value of `consensusTimestamp`. Subsequent reads from it will return its default value.
  public mutating func clearConsensusTimestamp() {_uniqueStorage()._consensusTimestamp = nil}

  ///*
  /// The ID of the transaction this record represents
  public var transactionID: Proto_TransactionID {
    get {return _storage._transactionID ?? Proto_TransactionID()}
    set {_uniqueStorage()._transactionID = newValue}
  }
  /// Returns true if `transactionID` has been explicitly set.
  public var hasTransactionID: Bool {return _storage._transactionID != nil}
  /// Clears the value of `transactionID`. Subsequent reads from it will return its default value.
  public mutating func clearTransactionID() {_uniqueStorage()._transactionID = nil}

  ///*
  /// The memo that was submitted as part of the transaction (max 100 bytes)
  public var memo: String {
    get {return _storage._memo}
    set {_uniqueStorage()._memo = newValue}
  }

  ///*
  /// The actual transaction fee charged, not the original transactionFee value from
  /// TransactionBody
  public var transactionFee: UInt64 {
    get {return _storage._transactionFee}
    set {_uniqueStorage()._transactionFee = newValue}
  }

  public var body: OneOf_Body? {
    get {return _storage._body}
    set {_uniqueStorage()._body = newValue}
  }

  ///*
  /// Record of the value returned by the smart contract function (if it completed and didn't
  /// fail) from ContractCallTransaction
  public var contractCallResult: Proto_ContractFunctionResult {
    get {
      if case .contractCallResult(let v)? = _storage._body {return v}
      return Proto_ContractFunctionResult()
    }
    set {_uniqueStorage()._body = .contractCallResult(newValue)}
  }

  ///*
  /// Record of the value returned by the smart contract constructor (if it completed and
  /// didn't fail) from ContractCreateTransaction
  public var contractCreateResult: Proto_ContractFunctionResult {
    get {
      if case .contractCreateResult(let v)? = _storage._body {return v}
      return Proto_ContractFunctionResult()
    }
    set {_uniqueStorage()._body = .contractCreateResult(newValue)}
  }

  ///*
  /// All hbar transfers as a result of this transaction, such as fees, or transfers performed by
  /// the transaction, or by a smart contract it calls, or by the creation of threshold records
  /// that it triggers.
  public var transferList: Proto_TransferList {
    get {return _storage._transferList ?? Proto_TransferList()}
    set {_uniqueStorage()._transferList = newValue}
  }
  /// Returns true if `transferList` has been explicitly set.
  public var hasTransferList: Bool {return _storage._transferList != nil}
  /// Clears the value of `transferList`. Subsequent reads from it will return its default value.
  public mutating func clearTransferList() {_uniqueStorage()._transferList = nil}

  ///*
  /// All Token transfers as a result of this transaction
  public var tokenTransferLists: [Proto_TokenTransferList] {
    get {return _storage._tokenTransferLists}
    set {_uniqueStorage()._tokenTransferLists = newValue}
  }

  ///*
  /// Reference to the scheduled transaction ID that this transaction record represent
  public var scheduleRef: Proto_ScheduleID {
    get {return _storage._scheduleRef ?? Proto_ScheduleID()}
    set {_uniqueStorage()._scheduleRef = newValue}
  }
  /// Returns true if `scheduleRef` has been explicitly set.
  public var hasScheduleRef: Bool {return _storage._scheduleRef != nil}
  /// Clears the value of `scheduleRef`. Subsequent reads from it will return its default value.
  public mutating func clearScheduleRef() {_uniqueStorage()._scheduleRef = nil}

  ///*
  /// All custom fees that were assessed during a CryptoTransfer, and must be paid if the
  /// transaction status resolved to SUCCESS
  public var assessedCustomFees: [Proto_AssessedCustomFee] {
    get {return _storage._assessedCustomFees}
    set {_uniqueStorage()._assessedCustomFees = newValue}
  }

  ///*
  /// All token associations implicitly created while handling this transaction
  public var automaticTokenAssociations: [Proto_TokenAssociation] {
    get {return _storage._automaticTokenAssociations}
    set {_uniqueStorage()._automaticTokenAssociations = newValue}
  }

  ///*
  /// In the record of an internal transaction, the consensus timestamp of the user
  /// transaction that spawned it.
  public var parentConsensusTimestamp: Proto_Timestamp {
    get {return _storage._parentConsensusTimestamp ?? Proto_Timestamp()}
    set {_uniqueStorage()._parentConsensusTimestamp = newValue}
  }
  /// Returns true if `parentConsensusTimestamp` has been explicitly set.
  public var hasParentConsensusTimestamp: Bool {return _storage._parentConsensusTimestamp != nil}
  /// Clears the value of `parentConsensusTimestamp`. Subsequent reads from it will return its default value.
  public mutating func clearParentConsensusTimestamp() {_uniqueStorage()._parentConsensusTimestamp = nil}

  ///*
  /// In the record of a CryptoCreate transaction triggered by a user transaction with a
  /// (previously unused) alias, the new account's alias.
  public var alias: Data {
    get {return _storage._alias}
    set {_uniqueStorage()._alias = newValue}
  }

  ///*
  /// The keccak256 hash of the ethereumData. This field will only be populated for
  /// EthereumTransaction.
  public var ethereumHash: Data {
    get {return _storage._ethereumHash}
    set {_uniqueStorage()._ethereumHash = newValue}
  }

  ///*
  /// List of accounts with the corresponding staking rewards paid as a result of a transaction.
  public var paidStakingRewards: [Proto_AccountAmount] {
    get {return _storage._paidStakingRewards}
    set {_uniqueStorage()._paidStakingRewards = newValue}
  }

  public var entropy: OneOf_Entropy? {
    get {return _storage._entropy}
    set {_uniqueStorage()._entropy = newValue}
  }

  ///*
  /// In the record of a UtilPrng transaction with no output range, a pseudorandom 384-bit string.
  public var prngBytes: Data {
    get {
      if case .prngBytes(let v)? = _storage._entropy {return v}
      return Data()
    }
    set {_uniqueStorage()._entropy = .prngBytes(newValue)}
  }

  ///*
  /// In the record of a PRNG transaction with an output range, the output of a PRNG whose input was a 384-bit string.
  public var prngNumber: Int32 {
    get {
      if case .prngNumber(let v)? = _storage._entropy {return v}
      return 0
    }
    set {_uniqueStorage()._entropy = .prngNumber(newValue)}
  }

  ///*
  /// The new default EVM address of the account created by this transaction.
  /// This field is populated only when the EVM address is not specified in the related transaction body.
  public var evmAddress: Data {
    get {return _storage._evmAddress}
    set {_uniqueStorage()._evmAddress = newValue}
  }

  public var unknownFields = SwiftProtobuf.UnknownStorage()

  public enum OneOf_Body: Equatable {
    ///*
    /// Record of the value returned by the smart contract function (if it completed and didn't
    /// fail) from ContractCallTransaction
    case contractCallResult(Proto_ContractFunctionResult)
    ///*
    /// Record of the value returned by the smart contract constructor (if it completed and
    /// didn't fail) from ContractCreateTransaction
    case contractCreateResult(Proto_ContractFunctionResult)

  #if !swift(>=4.1)
    public static func ==(lhs: Proto_TransactionRecord.OneOf_Body, rhs: Proto_TransactionRecord.OneOf_Body) -> Bool {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch (lhs, rhs) {
      case (.contractCallResult, .contractCallResult): return {
        guard case .contractCallResult(let l) = lhs, case .contractCallResult(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.contractCreateResult, .contractCreateResult): return {
        guard case .contractCreateResult(let l) = lhs, case .contractCreateResult(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      default: return false
      }
    }
  #endif
  }

  public enum OneOf_Entropy: Equatable {
    ///*
    /// In the record of a UtilPrng transaction with no output range, a pseudorandom 384-bit string.
    case prngBytes(Data)
    ///*
    /// In the record of a PRNG transaction with an output range, the output of a PRNG whose input was a 384-bit string.
    case prngNumber(Int32)

  #if !swift(>=4.1)
    public static func ==(lhs: Proto_TransactionRecord.OneOf_Entropy, rhs: Proto_TransactionRecord.OneOf_Entropy) -> Bool {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch (lhs, rhs) {
      case (.prngBytes, .prngBytes): return {
        guard case .prngBytes(let l) = lhs, case .prngBytes(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.prngNumber, .prngNumber): return {
        guard case .prngNumber(let l) = lhs, case .prngNumber(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      default: return false
      }
    }
  #endif
  }

  public init() {}

  fileprivate var _storage = _StorageClass.defaultInstance
}

#if swift(>=5.5) && canImport(_Concurrency)
extension Proto_TransactionRecord: @unchecked Sendable {}
extension Proto_TransactionRecord.OneOf_Body: @unchecked Sendable {}
extension Proto_TransactionRecord.OneOf_Entropy: @unchecked Sendable {}
#endif  // swift(>=5.5) && canImport(_Concurrency)

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "proto"

extension Proto_TransactionRecord: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  public static let protoMessageName: String = _protobuf_package + ".TransactionRecord"
  public static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "receipt"),
    2: .same(proto: "transactionHash"),
    3: .same(proto: "consensusTimestamp"),
    4: .same(proto: "transactionID"),
    5: .same(proto: "memo"),
    6: .same(proto: "transactionFee"),
    7: .same(proto: "contractCallResult"),
    8: .same(proto: "contractCreateResult"),
    10: .same(proto: "transferList"),
    11: .same(proto: "tokenTransferLists"),
    12: .same(proto: "scheduleRef"),
    13: .standard(proto: "assessed_custom_fees"),
    14: .standard(proto: "automatic_token_associations"),
    15: .standard(proto: "parent_consensus_timestamp"),
    16: .same(proto: "alias"),
    17: .standard(proto: "ethereum_hash"),
    18: .standard(proto: "paid_staking_rewards"),
    19: .standard(proto: "prng_bytes"),
    20: .standard(proto: "prng_number"),
    21: .standard(proto: "evm_address"),
  ]

  fileprivate class _StorageClass {
    var _receipt: Proto_TransactionReceipt? = nil
    var _transactionHash: Data = Data()
    var _consensusTimestamp: Proto_Timestamp? = nil
    var _transactionID: Proto_TransactionID? = nil
    var _memo: String = String()
    var _transactionFee: UInt64 = 0
    var _body: Proto_TransactionRecord.OneOf_Body?
    var _transferList: Proto_TransferList? = nil
    var _tokenTransferLists: [Proto_TokenTransferList] = []
    var _scheduleRef: Proto_ScheduleID? = nil
    var _assessedCustomFees: [Proto_AssessedCustomFee] = []
    var _automaticTokenAssociations: [Proto_TokenAssociation] = []
    var _parentConsensusTimestamp: Proto_Timestamp? = nil
    var _alias: Data = Data()
    var _ethereumHash: Data = Data()
    var _paidStakingRewards: [Proto_AccountAmount] = []
    var _entropy: Proto_TransactionRecord.OneOf_Entropy?
    var _evmAddress: Data = Data()

    static let defaultInstance = _StorageClass()

    private init() {}

    init(copying source: _StorageClass) {
      _receipt = source._receipt
      _transactionHash = source._transactionHash
      _consensusTimestamp = source._consensusTimestamp
      _transactionID = source._transactionID
      _memo = source._memo
      _transactionFee = source._transactionFee
      _body = source._body
      _transferList = source._transferList
      _tokenTransferLists = source._tokenTransferLists
      _scheduleRef = source._scheduleRef
      _assessedCustomFees = source._assessedCustomFees
      _automaticTokenAssociations = source._automaticTokenAssociations
      _parentConsensusTimestamp = source._parentConsensusTimestamp
      _alias = source._alias
      _ethereumHash = source._ethereumHash
      _paidStakingRewards = source._paidStakingRewards
      _entropy = source._entropy
      _evmAddress = source._evmAddress
    }
  }

  fileprivate mutating func _uniqueStorage() -> _StorageClass {
    if !isKnownUniquelyReferenced(&_storage) {
      _storage = _StorageClass(copying: _storage)
    }
    return _storage
  }

  public mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    _ = _uniqueStorage()
    try withExtendedLifetime(_storage) { (_storage: _StorageClass) in
      while let fieldNumber = try decoder.nextFieldNumber() {
        // The use of inline closures is to circumvent an issue where the compiler
        // allocates stack space for every case branch when no optimizations are
        // enabled. https://github.com/apple/swift-protobuf/issues/1034
        switch fieldNumber {
        case 1: try { try decoder.decodeSingularMessageField(value: &_storage._receipt) }()
        case 2: try { try decoder.decodeSingularBytesField(value: &_storage._transactionHash) }()
        case 3: try { try decoder.decodeSingularMessageField(value: &_storage._consensusTimestamp) }()
        case 4: try { try decoder.decodeSingularMessageField(value: &_storage._transactionID) }()
        case 5: try { try decoder.decodeSingularStringField(value: &_storage._memo) }()
        case 6: try { try decoder.decodeSingularUInt64Field(value: &_storage._transactionFee) }()
        case 7: try {
          var v: Proto_ContractFunctionResult?
          var hadOneofValue = false
          if let current = _storage._body {
            hadOneofValue = true
            if case .contractCallResult(let m) = current {v = m}
          }
          try decoder.decodeSingularMessageField(value: &v)
          if let v = v {
            if hadOneofValue {try decoder.handleConflictingOneOf()}
            _storage._body = .contractCallResult(v)
          }
        }()
        case 8: try {
          var v: Proto_ContractFunctionResult?
          var hadOneofValue = false
          if let current = _storage._body {
            hadOneofValue = true
            if case .contractCreateResult(let m) = current {v = m}
          }
          try decoder.decodeSingularMessageField(value: &v)
          if let v = v {
            if hadOneofValue {try decoder.handleConflictingOneOf()}
            _storage._body = .contractCreateResult(v)
          }
        }()
        case 10: try { try decoder.decodeSingularMessageField(value: &_storage._transferList) }()
        case 11: try { try decoder.decodeRepeatedMessageField(value: &_storage._tokenTransferLists) }()
        case 12: try { try decoder.decodeSingularMessageField(value: &_storage._scheduleRef) }()
        case 13: try { try decoder.decodeRepeatedMessageField(value: &_storage._assessedCustomFees) }()
        case 14: try { try decoder.decodeRepeatedMessageField(value: &_storage._automaticTokenAssociations) }()
        case 15: try { try decoder.decodeSingularMessageField(value: &_storage._parentConsensusTimestamp) }()
        case 16: try { try decoder.decodeSingularBytesField(value: &_storage._alias) }()
        case 17: try { try decoder.decodeSingularBytesField(value: &_storage._ethereumHash) }()
        case 18: try { try decoder.decodeRepeatedMessageField(value: &_storage._paidStakingRewards) }()
        case 19: try {
          var v: Data?
          try decoder.decodeSingularBytesField(value: &v)
          if let v = v {
            if _storage._entropy != nil {try decoder.handleConflictingOneOf()}
            _storage._entropy = .prngBytes(v)
          }
        }()
        case 20: try {
          var v: Int32?
          try decoder.decodeSingularInt32Field(value: &v)
          if let v = v {
            if _storage._entropy != nil {try decoder.handleConflictingOneOf()}
            _storage._entropy = .prngNumber(v)
          }
        }()
        case 21: try { try decoder.decodeSingularBytesField(value: &_storage._evmAddress) }()
        default: break
        }
      }
    }
  }

  public func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    try withExtendedLifetime(_storage) { (_storage: _StorageClass) in
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every if/case branch local when no optimizations
      // are enabled. https://github.com/apple/swift-protobuf/issues/1034 and
      // https://github.com/apple/swift-protobuf/issues/1182
      try { if let v = _storage._receipt {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 1)
      } }()
      if !_storage._transactionHash.isEmpty {
        try visitor.visitSingularBytesField(value: _storage._transactionHash, fieldNumber: 2)
      }
      try { if let v = _storage._consensusTimestamp {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 3)
      } }()
      try { if let v = _storage._transactionID {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 4)
      } }()
      if !_storage._memo.isEmpty {
        try visitor.visitSingularStringField(value: _storage._memo, fieldNumber: 5)
      }
      if _storage._transactionFee != 0 {
        try visitor.visitSingularUInt64Field(value: _storage._transactionFee, fieldNumber: 6)
      }
      switch _storage._body {
      case .contractCallResult?: try {
        guard case .contractCallResult(let v)? = _storage._body else { preconditionFailure() }
        try visitor.visitSingularMessageField(value: v, fieldNumber: 7)
      }()
      case .contractCreateResult?: try {
        guard case .contractCreateResult(let v)? = _storage._body else { preconditionFailure() }
        try visitor.visitSingularMessageField(value: v, fieldNumber: 8)
      }()
      case nil: break
      }
      try { if let v = _storage._transferList {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 10)
      } }()
      if !_storage._tokenTransferLists.isEmpty {
        try visitor.visitRepeatedMessageField(value: _storage._tokenTransferLists, fieldNumber: 11)
      }
      try { if let v = _storage._scheduleRef {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 12)
      } }()
      if !_storage._assessedCustomFees.isEmpty {
        try visitor.visitRepeatedMessageField(value: _storage._assessedCustomFees, fieldNumber: 13)
      }
      if !_storage._automaticTokenAssociations.isEmpty {
        try visitor.visitRepeatedMessageField(value: _storage._automaticTokenAssociations, fieldNumber: 14)
      }
      try { if let v = _storage._parentConsensusTimestamp {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 15)
      } }()
      if !_storage._alias.isEmpty {
        try visitor.visitSingularBytesField(value: _storage._alias, fieldNumber: 16)
      }
      if !_storage._ethereumHash.isEmpty {
        try visitor.visitSingularBytesField(value: _storage._ethereumHash, fieldNumber: 17)
      }
      if !_storage._paidStakingRewards.isEmpty {
        try visitor.visitRepeatedMessageField(value: _storage._paidStakingRewards, fieldNumber: 18)
      }
      switch _storage._entropy {
      case .prngBytes?: try {
        guard case .prngBytes(let v)? = _storage._entropy else { preconditionFailure() }
        try visitor.visitSingularBytesField(value: v, fieldNumber: 19)
      }()
      case .prngNumber?: try {
        guard case .prngNumber(let v)? = _storage._entropy else { preconditionFailure() }
        try visitor.visitSingularInt32Field(value: v, fieldNumber: 20)
      }()
      case nil: break
      }
      if !_storage._evmAddress.isEmpty {
        try visitor.visitSingularBytesField(value: _storage._evmAddress, fieldNumber: 21)
      }
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  public static func ==(lhs: Proto_TransactionRecord, rhs: Proto_TransactionRecord) -> Bool {
    if lhs._storage !== rhs._storage {
      let storagesAreEqual: Bool = withExtendedLifetime((lhs._storage, rhs._storage)) { (_args: (_StorageClass, _StorageClass)) in
        let _storage = _args.0
        let rhs_storage = _args.1
        if _storage._receipt != rhs_storage._receipt {return false}
        if _storage._transactionHash != rhs_storage._transactionHash {return false}
        if _storage._consensusTimestamp != rhs_storage._consensusTimestamp {return false}
        if _storage._transactionID != rhs_storage._transactionID {return false}
        if _storage._memo != rhs_storage._memo {return false}
        if _storage._transactionFee != rhs_storage._transactionFee {return false}
        if _storage._body != rhs_storage._body {return false}
        if _storage._transferList != rhs_storage._transferList {return false}
        if _storage._tokenTransferLists != rhs_storage._tokenTransferLists {return false}
        if _storage._scheduleRef != rhs_storage._scheduleRef {return false}
        if _storage._assessedCustomFees != rhs_storage._assessedCustomFees {return false}
        if _storage._automaticTokenAssociations != rhs_storage._automaticTokenAssociations {return false}
        if _storage._parentConsensusTimestamp != rhs_storage._parentConsensusTimestamp {return false}
        if _storage._alias != rhs_storage._alias {return false}
        if _storage._ethereumHash != rhs_storage._ethereumHash {return false}
        if _storage._paidStakingRewards != rhs_storage._paidStakingRewards {return false}
        if _storage._entropy != rhs_storage._entropy {return false}
        if _storage._evmAddress != rhs_storage._evmAddress {return false}
        return true
      }
      if !storagesAreEqual {return false}
    }
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}
