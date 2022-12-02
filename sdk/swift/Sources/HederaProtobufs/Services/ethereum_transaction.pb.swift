// DO NOT EDIT.
// swift-format-ignore-file
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: ethereum_transaction.proto
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

public struct Proto_EthereumTransactionBody {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  ///*
  /// The raw Ethereum transaction (RLP encoded type 0, 1, and 2). Complete
  /// unless the callData field is set.
  public var ethereumData: Data = Data()

  ///*
  /// For large transactions (for example contract create) this is the callData
  /// of the ethereumData. The data in the ethereumData will be re-written with
  /// the callData element as a zero length string with the original contents in
  /// the referenced file at time of execution. The ethereumData will need to be
  /// "rehydrated" with the callData for signature validation to pass.
  public var callData: Proto_FileID {
    get {return _callData ?? Proto_FileID()}
    set {_callData = newValue}
  }
  /// Returns true if `callData` has been explicitly set.
  public var hasCallData: Bool {return self._callData != nil}
  /// Clears the value of `callData`. Subsequent reads from it will return its default value.
  public mutating func clearCallData() {self._callData = nil}

  ///*
  /// The maximum amount, in tinybars, that the payer of the hedera transaction
  /// is willing to pay to complete the transaction.
  ///
  /// Ordinarily the account with the ECDSA alias corresponding to the public
  /// key that is extracted from the ethereum_data signature is responsible for
  /// fees that result from the execution of the transaction. If that amount of
  /// authorized fees is not sufficient then the payer of the transaction can be
  /// charged, up to but not exceeding this amount. If the ethereum_data
  /// transaction authorized an amount that was insufficient then the payer will
  /// only be charged the amount needed to make up the difference. If the gas
  /// price in the transaction was set to zero then the payer will be assessed
  /// the entire fee.
  public var maxGasAllowance: Int64 = 0

  public var unknownFields = SwiftProtobuf.UnknownStorage()

  public init() {}

  fileprivate var _callData: Proto_FileID? = nil
}

#if swift(>=5.5) && canImport(_Concurrency)
extension Proto_EthereumTransactionBody: @unchecked Sendable {}
#endif  // swift(>=5.5) && canImport(_Concurrency)

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "proto"

extension Proto_EthereumTransactionBody: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  public static let protoMessageName: String = _protobuf_package + ".EthereumTransactionBody"
  public static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "ethereum_data"),
    2: .standard(proto: "call_data"),
    3: .standard(proto: "max_gas_allowance"),
  ]

  public mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.ethereumData) }()
      case 2: try { try decoder.decodeSingularMessageField(value: &self._callData) }()
      case 3: try { try decoder.decodeSingularInt64Field(value: &self.maxGasAllowance) }()
      default: break
      }
    }
  }

  public func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    // The use of inline closures is to circumvent an issue where the compiler
    // allocates stack space for every if/case branch local when no optimizations
    // are enabled. https://github.com/apple/swift-protobuf/issues/1034 and
    // https://github.com/apple/swift-protobuf/issues/1182
    if !self.ethereumData.isEmpty {
      try visitor.visitSingularBytesField(value: self.ethereumData, fieldNumber: 1)
    }
    try { if let v = self._callData {
      try visitor.visitSingularMessageField(value: v, fieldNumber: 2)
    } }()
    if self.maxGasAllowance != 0 {
      try visitor.visitSingularInt64Field(value: self.maxGasAllowance, fieldNumber: 3)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  public static func ==(lhs: Proto_EthereumTransactionBody, rhs: Proto_EthereumTransactionBody) -> Bool {
    if lhs.ethereumData != rhs.ethereumData {return false}
    if lhs._callData != rhs._callData {return false}
    if lhs.maxGasAllowance != rhs.maxGasAllowance {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}
