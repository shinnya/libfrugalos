//! test

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::branch::Branch3;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5, F6};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof, Optional,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    BoolDecoder, BoolEncoder, StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder,
    Uint64Decoder, Uint64Encoder,
};

use consistency::ReadConsistency;
use entity::object::ObjectVersion;
use expect::Expect;
use protobuf::bucket::{BucketIdDecoder, BucketIdEncoder};
use protobuf::consistency::{ReadConsistencyDecoder, ReadConsistencyEncoder};
use protobuf::deadline::{decode_deadline, encode_deadline, DeadlineDecoder, DeadlineEncoder};
use protobuf::expect::{ExpectDecoder, ExpectEncoder};
use protobuf::object::{
    ObjectIdDecoder, ObjectIdEncoder, ObjectVersionDecoder, ObjectVersionEncoder,
    ObjectVersionsDecoder,
};
use schema::frugalos::{
    CountFragmentsRequest, HeadObjectRequest, ListObjectsRequest, ObjectRequest, SegmentRequest,
    VersionRequest,
};

/// Decoder for `ObjectRequest`.
#[derive(Debug)]
pub struct ObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeadlineDecoder>>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            Optional<MessageFieldDecoder<F5, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ObjectRequestDecoder, ObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.2))?;
    Ok(ObjectRequest {
        bucket_id: t.0.clone(),
        object_id: t.1.clone(),
        deadline,
        expect: t.3,
        consistency: t.4,
    })
});

/// Encoder for `ObjectRequest`.
#[derive(Debug)]
pub struct ObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, StringEncoder>,
            FieldEncoder<F3, DeadlineEncoder>,
            MessageFieldEncoder<F4, PreEncode<ExpectEncoder>>,
            Optional<MessageFieldEncoder<F5, ReadConsistencyEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(ObjectRequestEncoder, ObjectRequest, |item: Self::Item| {
    (
        item.bucket_id,
        item.object_id,
        encode_deadline(item.deadline),
        item.expect,
        item.consistency,
    )
});

/// Decoder for `CountFragmentsRequest`.
#[derive(Debug)]
pub struct CountFragmentsRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectIdDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeadlineDecoder>>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            MessageFieldDecoder<F5, ReadConsistencyDecoder>,
        )>,
    >,
}
impl_message_decode!(
    CountFragmentsRequestDecoder,
    CountFragmentsRequest,
    |t: (String, String, _, _, _,)| {
        let deadline = track!(decode_deadline(t.2))?;
        Ok(CountFragmentsRequest {
            bucket_id: t.0.clone(),
            object_id: t.1.clone(),
            deadline,
            expect: t.3,
            consistency: t.4,
        })
    }
);

/// Encoder for `CountFragmentsRequest`.
#[derive(Debug)]
pub struct CountFragmentsRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, ObjectIdEncoder>,
            FieldEncoder<F3, DeadlineEncoder>,
            MessageFieldEncoder<F4, PreEncode<ExpectEncoder>>,
            MessageFieldEncoder<F5, ReadConsistencyEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    CountFragmentsRequestEncoder,
    CountFragmentsRequest,
    |item: Self::Item| {
        (
            item.bucket_id,
            item.object_id,
            encode_deadline(item.deadline),
            item.expect,
            item.consistency,
        )
    }
);

/// Decoder for `HeadObjectRequest`.
#[derive(Debug)]
pub struct HeadObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, ObjectIdDecoder>>,
            MaybeDefault<FieldDecoder<F3, DeadlineDecoder>>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            MessageFieldDecoder<F5, ReadConsistencyDecoder>,
            MaybeDefault<FieldDecoder<F6, BoolDecoder>>,
        )>,
    >,
}
impl_message_decode!(HeadObjectRequestDecoder, HeadObjectRequest, |t: (
    String,
    String,
    _,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.2))?;
    Ok(HeadObjectRequest {
        bucket_id: t.0.clone(),
        object_id: t.1.clone(),
        deadline,
        expect: t.3,
        consistency: t.4,
        check_storage: t.5,
    })
});

/// Encoder for `HeadObjectRequest`.
#[derive(Debug)]
pub struct HeadObjectRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, ObjectIdEncoder>,
            FieldEncoder<F3, DeadlineEncoder>,
            MessageFieldEncoder<F4, PreEncode<ExpectEncoder>>,
            MessageFieldEncoder<F5, ReadConsistencyEncoder>,
            FieldEncoder<F6, BoolEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    HeadObjectRequestEncoder,
    HeadObjectRequest,
    |item: Self::Item| {
        (
            item.bucket_id,
            item.object_id,
            encode_deadline(item.deadline),
            item.expect,
            item.consistency,
            item.check_storage,
        )
    }
);

/// Decoder for `VersionRequest`.
#[derive(Debug)]
pub struct VersionRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, ObjectVersionDecoder>>,
            MaybeDefault<FieldDecoder<F4, DeadlineDecoder>>,
        )>,
    >,
}
impl_message_decode!(VersionRequestDecoder, VersionRequest, |t: (
    String,
    _,
    _,
    _,
)| {
    let deadline = track!(decode_deadline(t.3))?;
    Ok(VersionRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
        object_version: ObjectVersion(t.2),
        deadline,
    })
});

/// Encoder for `VersionRequest`.
#[derive(Debug)]
pub struct VersionRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F4, ObjectVersionEncoder>,
            FieldEncoder<F4, DeadlineEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(VersionRequestEncoder, VersionRequest, |item: Self::Item| {
    (
        item.bucket_id,
        item.segment as u32,
        item.object_version.0,
        encode_deadline(item.deadline),
    )
});

/// Decoder for `ListObjectsRequestEncoder`.
#[derive(Debug)]
pub struct ListObjectsRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<MessageFieldDecoder<F3, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ListObjectsRequestDecoder, ListObjectsRequest, |t: (
    String,
    _,
    _
)| {
    Ok(ListObjectsRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
        consistency: t.2,
    })
});

/// Encoder for `ListObjectsRequestEncoder`.
#[derive(Debug)]
pub struct ListObjectsRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    ListObjectsRequestEncoder,
    ListObjectsRequest,
    |t: Self::Item| { (t.bucket_id, t.segment as u32) }
);

/// Decoder for `SegmentRequest`.
#[derive(Debug)]
pub struct SegmentRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            FieldDecoder<F1, BucketIdDecoder>,
            FieldDecoder<F2, Uint32Decoder>,
        )>,
    >,
}
impl_message_decode!(SegmentRequestDecoder, SegmentRequest, |t: (String, u32)| {
    Ok(SegmentRequest {
        bucket_id: t.0.clone(),
        segment: t.1 as u16,
    })
});

/// Encoder for `SegmentRequest`.
#[derive(Debug)]
pub struct SegmentRequestEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, BucketIdEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(SegmentRequestEncoder, SegmentRequest, |t: Self::Item| {
    (t.bucket_id, t.segment as u32)
});
