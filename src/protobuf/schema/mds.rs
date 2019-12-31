//! test

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Optional,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    BytesDecoder, BytesEncoder, StringDecoder, StringEncoder, Uint64Decoder, Uint64Encoder,
};
use std::time::Duration;

use entity::node::LocalNodeId;
use entity::object::{DeleteObjectsByPrefixSummary, Metadata, ObjectSummary, ObjectVersion};
use protobuf::consistency::{ReadConsistencyDecoder, ReadConsistencyEncoder};
use protobuf::entity::node::{LocalNodeIdDecoder, LocalNodeIdEncoder};
use protobuf::entity::object::{
    DeleteObjectsByPrefixSummaryDecoder, DeleteObjectsByPrefixSummaryEncoder, MetadataDecoder,
    MetadataEncoder, ObjectPrefixDecoder, ObjectPrefixEncoder, ObjectRangeDecoder,
    ObjectRangeEncoder, ObjectSummaryDecoder, ObjectSummaryEncoder, ObjectVersionDecoder,
    ObjectVersionEncoder,
};
use protobuf::expect::{ExpectDecoder, ExpectEncoder};
use protobuf::{
    OptionDecoder, OptionEncoder, ResultDecoder, ResultEncoder, VecDecoder, VecEncoder,
};
use schema::mds::{
    ListObjectsRequest, ObjectCountRequest, ObjectRequest, PrefixRequest, PutObjectRequest,
    RangeRequest, VersionRequest,
};
use Result;

/// Decoder for `ObjectRequest`.
#[derive(Debug, Default)]
pub struct ObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MessageFieldDecoder<F3, ExpectDecoder>,
            Optional<MessageFieldDecoder<F4, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ObjectRequestDecoder, ObjectRequest, |t: (
    String,
    String,
    _,
    _,
)| {
    Ok(ObjectRequest {
        node_id: t.0.clone(),
        object_id: t.1.clone(),
        expect: t.2,
        consistency: t.3,
    })
});

/// Encoder for `ObjectRequest`.
#[derive(Debug, Default)]
pub struct ObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            FieldEncoder<F2, StringEncoder>,
            MessageFieldEncoder<F3, PreEncode<ExpectEncoder>>,
            Optional<MessageFieldEncoder<F4, ReadConsistencyEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(ObjectRequestEncoder, ObjectRequest, |item: Self::Item| {
    (item.node_id, item.object_id, item.expect, item.consistency)
});

/// Decoder for `ListObjectsRequest`.
#[derive(Debug, Default)]
pub struct ListObjectsRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ReadConsistencyDecoder>,
        )>,
    >,
}
impl_message_decode!(ListObjectsRequestDecoder, ListObjectsRequest, |t: (
    String,
    _,
)| {
    Ok(ListObjectsRequest {
        node_id: t.0.clone(),
        consistency: t.1,
    })
});

/// Encoder for `ListObjectsRequest`.
#[derive(Debug, Default)]
pub struct ListObjectsRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ReadConsistencyEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ListObjectsRequestEncoder,
    ListObjectsRequest,
    |item: Self::Item| { (item.node_id, item.consistency,) }
);

/// Decoder for `ObjectCountRequest`.
#[derive(Debug, Default)]
pub struct ObjectCountRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ReadConsistencyDecoder>,
        )>,
    >,
}
impl_message_decode!(ObjectCountRequestDecoder, ObjectCountRequest, |t: (
    String,
    _,
)| {
    Ok(ObjectCountRequest {
        node_id: t.0.clone(),
        consistency: t.1,
    })
});

/// Encoder for `ObjectCountRequest`.
#[derive(Debug, Default)]
pub struct ObjectCountRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ReadConsistencyEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ObjectCountRequestEncoder,
    ObjectCountRequest,
    |item: Self::Item| { (item.node_id, item.consistency,) }
);

/// Decoder for `VersionRequest`.
#[derive(Debug, Default)]
pub struct VersionRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectVersionDecoder>>,
        )>,
    >,
}
impl_message_decode!(VersionRequestDecoder, VersionRequest, |t: (String, _,)| {
    Ok(VersionRequest {
        node_id: t.0.clone(),
        object_version: ObjectVersion(t.1),
    })
});

/// Encoder for `VersionRequest`.
#[derive(Debug, Default)]
pub struct VersionRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            FieldEncoder<F2, ObjectVersionEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(VersionRequestEncoder, VersionRequest, |item: Self::Item| {
    (item.node_id, item.object_version.0)
});

/// Decoder for `RangeRequest`.
#[derive(Debug, Default)]
pub struct RangeRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ObjectRangeDecoder>,
        )>,
    >,
}
impl_message_decode!(RangeRequestDecoder, RangeRequest, |t: (String, _,)| {
    Ok(RangeRequest {
        node_id: t.0.clone(),
        targets: t.1,
    })
});

/// Encoder for `RangeRequest`.
#[derive(Debug, Default)]
pub struct RangeRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ObjectRangeEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(RangeRequestEncoder, RangeRequest, |item: Self::Item| {
    (item.node_id, item.targets)
});

/// Decoder for `PrefixRequest`.
#[derive(Debug, Default)]
pub struct PrefixRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MessageFieldDecoder<F2, ObjectPrefixDecoder>,
        )>,
    >,
}
impl_message_decode!(PrefixRequestDecoder, PrefixRequest, |t: (String, _,)| {
    Ok(PrefixRequest {
        node_id: t.0.clone(),
        prefix: t.1,
    })
});

/// Encoder for `PrefixRequest`.
#[derive(Debug, Default)]
pub struct PrefixRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            MessageFieldEncoder<F2, ObjectPrefixEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(PrefixRequestEncoder, PrefixRequest, |item: Self::Item| {
    (item.node_id, item.prefix)
});

/// Decoder for `PutObjectRequest`.
#[derive(Debug, Default)]
pub struct PutObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, LocalNodeIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MaybeDefault<FieldDecoder<F3, BytesDecoder>>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            FieldDecoder<F5, Uint64Decoder>, // TODO
        )>,
    >,
}
impl_message_decode!(PutObjectRequestDecoder, PutObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
)| {
    Ok(PutObjectRequest {
        node_id: t.0.clone(),
        object_id: t.1.clone(),
        metadata: t.2,
        expect: t.3,
        put_content_timeout: Duration::from_secs(t.4),
    })
});

/// Encoder for `PutObjectRequest`.
#[derive(Debug, Default)]
pub struct PutObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, LocalNodeIdEncoder>,
            FieldEncoder<F2, StringEncoder>,
            FieldEncoder<F3, BytesEncoder>,
            MessageFieldEncoder<F4, ExpectEncoder>,
            FieldEncoder<F5, Uint64Encoder>, // TODO
        )>,
    >,
}
impl_sized_message_encode!(
    PutObjectRequestEncoder,
    PutObjectRequest,
    |item: Self::Item| {
        (
            item.node_id,
            item.object_id,
            item.metadata,
            item.expect,
            item.put_content_timeout.as_secs(),
        )
    }
);

/// Decoder for a response of [GetLeaderRpc].
#[derive(Debug, Default)]
pub struct GetLeaderRpcResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<MessageDecoder<FieldDecoder<F1, LocalNodeIdDecoder>>>,
        >,
    >,
}
impl_message_decode!(GetLeaderRpcResponseDecoder, Result<LocalNodeId>, |t: _| Ok(
    t
));

/// Encoder for a response of [GetLeaderRpc].
#[derive(Debug, Default)]
pub struct GetLeaderRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<MessageEncoder<FieldEncoder<F1, LocalNodeIdEncoder>>>,
        >,
    >,
}
impl_message_encode!(
    GetLeaderRpcResponseEncoder,
    Result<LocalNodeId>,
    |item: Self::Item| item
);

/// Decoder for a response of `ListObjectsRpc`.
#[derive(Debug, Default)]
pub struct ListObjectsRpcResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<ObjectSummaryDecoder>>>>,
}
impl_message_decode!(
    ListObjectsRpcResponseDecoder,
    Result<Vec<ObjectSummary>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `ListObjectsRpc`.
#[derive(Debug, Default)]
pub struct ListObjectsRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<ObjectSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    ListObjectsRpcResponseEncoder,
    Result<Vec<ObjectSummary>>,
    |item: Self::Item| item
);

/// Decoder for a response of `GetObjectRpc`.
#[derive(Debug, Default)]
pub struct GetObjectRpcResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<MetadataDecoder>>>>,
}
impl_message_decode!(
    GetObjectRpcResponseDecoder,
    Result<Option<Metadata>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `GetObjectRpc`.
#[derive(Debug, Default)]
pub struct GetObjectRpcResponseEncoder {
    inner: MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<MetadataEncoder>>>>,
}
impl_message_encode!(
    GetObjectRpcResponseEncoder,
    Result<Option<Metadata>>,
    |item: Self::Item| item
);

/// Decoder for a response of `HeadObjectRpc`.
#[derive(Debug, Default)]
pub struct HeadObjectRpcResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<OptionDecoder<MessageDecoder<FieldDecoder<F1, ObjectVersionDecoder>>>>,
        >,
    >,
}
impl_message_decode!(
    HeadObjectRpcResponseDecoder,
    Result<Option<ObjectVersion>>,
    |t: Result<Option<u64>>| Ok(t.map(|x| x.map(ObjectVersion)))
);

/// Encoder for a response of `HeadObjectRpc`.
#[derive(Debug, Default)]
pub struct HeadObjectRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<OptionEncoder<MessageEncoder<FieldEncoder<F1, ObjectVersionEncoder>>>>,
        >,
    >,
}
impl_message_encode!(
    HeadObjectRpcResponseEncoder,
    Result<Option<ObjectVersion>>,
    |item: Self::Item| item.map(|x| x.map(|v| v.0))
);

/// Decoder for a response of `DeleteObjectRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectRpcResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<OptionDecoder<MessageDecoder<FieldDecoder<F1, ObjectVersionDecoder>>>>,
        >,
    >,
}
impl_message_decode!(
    DeleteObjectRpcResponseDecoder,
    Result<Option<ObjectVersion>>,
    |t: Result<Option<u64>>| Ok(t.map(|x| x.map(ObjectVersion)))
);

/// Encoder for a response of `DeleteObjectRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<OptionEncoder<MessageEncoder<FieldEncoder<F1, ObjectVersionEncoder>>>>,
        >,
    >,
}
impl_message_encode!(
    DeleteObjectRpcResponseEncoder,
    Result<Option<ObjectVersion>>,
    |item: Self::Item| item.map(|x| x.map(|v| v.0))
);

/// Decoder for a response of `GetLatestVersionRpc`.
#[derive(Debug, Default)]
pub struct GetLatestVersionRpcResponseDecoder {
    inner:
        MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<OptionDecoder<ObjectSummaryDecoder>>>>,
}
impl_message_decode!(
    GetLatestVersionRpcResponseDecoder,
    Result<Option<ObjectSummary>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `GetLatestVersionRpc`.
#[derive(Debug, Default)]
pub struct GetLatestVersionRpcResponseEncoder {
    inner:
        MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<OptionEncoder<ObjectSummaryEncoder>>>>,
}
impl_message_encode!(
    GetLatestVersionRpcResponseEncoder,
    Result<Option<ObjectSummary>>,
    |item: Self::Item| item
);

/// Decoder for a response of `DeleteObjectByVersionRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectByVersionRpcResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<
            F1,
            ResultDecoder<OptionDecoder<MessageDecoder<FieldDecoder<F1, ObjectVersionDecoder>>>>,
        >,
    >,
}
impl_message_decode!(
    DeleteObjectByVersionRpcResponseDecoder,
    Result<Option<ObjectVersion>>,
    |t: Result<Option<u64>>| Ok(t.map(|x| x.map(ObjectVersion)))
);

/// Encoder for a response of `DeleteObjectByVersionRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectByVersionRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<
            F1,
            ResultEncoder<OptionEncoder<MessageEncoder<FieldEncoder<F1, ObjectVersionEncoder>>>>,
        >,
    >,
}
impl_message_encode!(
    DeleteObjectByVersionRpcResponseEncoder,
    Result<Option<ObjectVersion>>,
    |item: Self::Item| item.map(|x| x.map(|v| v.0))
);

/// Decoder for a response of `DeleteObjectsByRangeRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByRangeRpcResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<ObjectSummaryDecoder>>>>,
}
impl_message_decode!(
    DeleteObjectsByRangeRpcResponseDecoder,
    Result<Vec<ObjectSummary>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `DeleteObjectsByRangeRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByRangeRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<ObjectSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    DeleteObjectsByRangeRpcResponseEncoder,
    Result<Vec<ObjectSummary>>,
    |item: Self::Item| item
);

/// Decoder for a response of `GetObjectCountRpc`.
#[derive(Debug, Default)]
pub struct GetObjectCountRpcResponseDecoder {
    inner: MessageDecoder<
        MessageFieldDecoder<F1, ResultDecoder<MessageDecoder<FieldDecoder<F1, Uint64Decoder>>>>,
    >,
}
impl_message_decode!(GetObjectCountRpcResponseDecoder, Result<u64>, |t: _| Ok(t));

/// Encoder for a response of `GetObjectCountRpc`.
#[derive(Debug, Default)]
pub struct GetObjectCountRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, ResultEncoder<MessageEncoder<FieldEncoder<F1, Uint64Encoder>>>>,
    >,
}
impl_message_encode!(
    GetObjectCountRpcResponseEncoder,
    Result<u64>,
    |item: Self::Item| item
);

/// Decoder for a response of `DeleteObjectsByPrefixRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByPrefixRpcResponseDecoder {
    inner:
        MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<DeleteObjectsByPrefixSummaryDecoder>>>,
}
impl_message_decode!(
    DeleteObjectsByPrefixRpcResponseDecoder,
    Result<DeleteObjectsByPrefixSummary>,
    |t: _| Ok(t)
);

/// Encoder for a response of `DeleteObjectsByPrefixRpc`.
#[derive(Debug, Default)]
pub struct DeleteObjectsByPrefixRpcResponseEncoder {
    inner:
        MessageEncoder<MessageFieldEncoder<F1, ResultEncoder<DeleteObjectsByPrefixSummaryEncoder>>>,
}
impl_message_encode!(
    DeleteObjectsByPrefixRpcResponseEncoder,
    Result<DeleteObjectsByPrefixSummary>,
    |item: Self::Item| item
);

/// Decoder for a response of `ListObjectsByPrefixRpc`.
#[derive(Debug, Default)]
pub struct ListObjectsByPrefixRpcResponseDecoder {
    inner: MessageDecoder<MessageFieldDecoder<F1, ResultDecoder<VecDecoder<ObjectSummaryDecoder>>>>,
}
impl_message_decode!(
    ListObjectsByPrefixRpcResponseDecoder,
    Result<Vec<ObjectSummary>>,
    |t: _| Ok(t)
);

/// Encoder for a response of `ListObjectsByPrefixRpc`.
#[derive(Debug, Default)]
pub struct ListObjectsByPrefixRpcResponseEncoder {
    inner: MessageEncoder<
        MessageFieldEncoder<F1, PreEncode<ResultEncoder<VecEncoder<ObjectSummaryEncoder>>>>,
    >,
}
impl_message_encode!(
    ListObjectsByPrefixRpcResponseEncoder,
    Result<Vec<ObjectSummary>>,
    |item: Self::Item| item
);
