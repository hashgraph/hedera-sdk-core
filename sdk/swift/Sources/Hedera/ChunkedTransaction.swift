import CHedera
import Foundation

public class ChunkedTransaction: Transaction {
    internal static let defaultMaxChunks = 20
    internal static let defaultChunkSize = 1024

    public override init() {
        super.init()
    }

    internal init(data: Data) {
        self.data = data
        super.init()
    }

    public required init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        data = try container.decodeIfPresent(.data).map(Data.base64Encoded) ?? Data()
        maxChunks = try container.decodeIfPresent(.maxChunks) ?? Self.defaultMaxChunks
        chunkSize = try container.decodeIfPresent(.chunkSize) ?? Self.defaultChunkSize

        try super.init(from: decoder)
    }

    /// Message/contents for this transaction.
    ///
    /// Please expose this under the appropriate name (with the frozen check) when inheriting from this class.
    internal final var data: Data = Data() {
        willSet {
            // note: This exists in case one is forgotten, prefer the `willSet` elsewhere.
            ensureNotFrozen(fieldName: "data")
        }
    }

    /// The maximum number of chunks this transaction will be split into.
    public final var maxChunks = defaultMaxChunks {
        willSet {
            ensureNotFrozen(fieldName: "maxChunks")
        }
    }

    /// Sets the maximum number of chunks this transaction will be split into.
    @discardableResult
    public final func maxChunks(_ maxChunks: Int) -> Self {
        self.maxChunks = maxChunks

        return self
    }

    /// The maximum size of any chunk of this transaction.
    public final var chunkSize = defaultChunkSize {
        willSet {
            ensureNotFrozen(fieldName: "chunkSize")
        }
    }

    /// Sets the maximum size of any chunk of this transaction.
    @discardableResult
    public final func chunkSize(_ chunkSize: Int) -> Self {
        precondition(chunkSize != 0)

        self.chunkSize = chunkSize

        return self
    }

    internal final var usedChunks: Int {
        if data.isEmpty {
            return 1
        }

        // div ceil algorithm
        return (data.count + chunkSize) / chunkSize
    }

    internal var waitForReceipt: Bool { false }

    private enum CodingKeys: CodingKey {
        case data
        case maxChunks
        case chunkSize
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encode(data, forKey: .data)

        if maxChunks != Self.defaultMaxChunks {
            try container.encode(maxChunks, forKey: .maxChunks)
        }

        if chunkSize != Self.defaultChunkSize {
            try container.encode(chunkSize, forKey: .chunkSize)
        }

        try super.encode(to: encoder)
    }

    internal final override func addSignatureSigner(_ signer: Signer) {
        precondition(
            self.usedChunks <= 1,
            "cannot manually add a signature to a chunked transaction with multiple chunks (message length `\(data.count)` > chunk size `\(chunkSize)`)"
        )

        super.addSignatureSigner(signer)
    }

    public final override func execute(_ client: Client, _ timeout: TimeInterval? = nil) async throws
        -> TransactionResponse
    {
        try await executeAll(client, timeout)[0]
    }

    public final func executeAll(_ client: Client, _ timeoutPerChunk: TimeInterval? = nil) async throws
        -> [TransactionResponse]
    {
        try freezeWith(client)

        return try await executeAllInternal(client, timeoutPerChunk)
    }

    private final func executeAllInternal(_ client: Client, _ timeoutPerChunk: TimeInterval? = nil) async throws
        -> [TransactionResponse]
    {
        try freezeWith(client)

        if let sources = sources {
            return try await SourceTransaction(inner: self, sources: sources).executeAll(
                client, timeoutPerChunk: timeoutPerChunk)
        }

        // encode self as a JSON request to pass to Rust
        let requestBytes = try JSONEncoder().encode(self)

        let request = String(data: requestBytes, encoding: .utf8)!

        return try await executeAllEncoded(client, request: request, timeoutPerChunk: timeoutPerChunk)
    }
}
