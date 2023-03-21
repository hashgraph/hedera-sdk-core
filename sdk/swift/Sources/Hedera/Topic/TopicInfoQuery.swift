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

/// Retrieve the latest state of a topic.
public final class TopicInfoQuery: Query<TopicInfo> {
    /// Create a new `TopicInfoQuery`.
    public init(
        topicId: TopicId? = nil
    ) {
        self.topicId = topicId
    }

    /// The topic ID for which information is requested.
    public var topicId: TopicId?

    /// Sets the topic ID for which information is requested.
    @discardableResult
    public func topicId(_ topicId: TopicId) -> Self {
        self.topicId = topicId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case topicId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(topicId, forKey: .topicId)

        try super.encode(to: encoder)
    }

    internal override func toQueryProtobufWith(_ header: Proto_QueryHeader) -> Proto_Query {
        .with { proto in
            proto.consensusGetTopicInfo = .with { proto in
                proto.header = header
                topicId?.toProtobufInto(&proto.topicID)
            }
        }
    }

    internal override func queryExecute(_ channel: GRPCChannel, _ request: Proto_Query) async throws -> Proto_Response {
        try await Proto_ConsensusServiceAsyncClient(channel: channel).getTopicInfo(request)
    }

    internal override func validateChecksums(on ledgerId: LedgerId) throws {
        try topicId?.validateChecksums(on: ledgerId)
        try super.validateChecksums(on: ledgerId)
    }
}
