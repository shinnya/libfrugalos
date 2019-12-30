//! test

use protobuf_codec::field::branch::Branch3;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof, Optional,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder, Uint64Decoder, Uint64Encoder,
};

use protobuf::bucket::{BucketIdDecoder, BucketIdEncoder};
use protobuf::consistency::{ReadConsistencyDecoder, ReadConsistencyEncoder};
use protobuf::expect::{ExpectDecoder, ExpectEncoder};
use consistency::ReadConsistency;
use expect::Expect;
use schema::frugalos::{
    ListObjectsRequest,
    ObjectRequest,
    SegmentRequest,
};

/// Decoder for `ObjectRequestDecoder`.
#[derive(Debug)]
pub struct ObjectRequestDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, BucketIdDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, Uint32Decoder>>,
            MessageFieldDecoder<F4, ExpectDecoder>,
            Optional<MessageFieldDecoder<F5, ReadConsistencyDecoder>>,
        )>,
    >,
}
impl_message_decode!(ObjectRequestDecoder, ObjectRequest, |t: (String, u32, u32, Expect, Option<ReadConsistency>)| {
// TODO
    Ok(ObjectRequest {
        bucket_id: t.0.clone(),
        object_id: Default::default(),
        deadline: Default::default(),
        expect: t.3,
        consistency: Default::default(),
    })
});

///// Encoder for `ObjectRequestEncoder`.
//#[derive(Debug)]
//pub struct ObjectRequestEncoder {
//    inner: MessageEncoder<
//        Fields<(
//            FieldEncoder<F1, BucketIdEncoder>,
//            FieldEncoder<F2, Uint32Encoder>,
//            FieldEncoder<F3, Uint32Encoder>,
//            MessageFieldEncoder<F4, ExpectEncoder>,
//            Optional<MessageFieldEncoder<F5, ReadConsistencyEncoder>>,
//        )>,
//    >,
//}
//impl_message_encode!(ObjectRequestEncoder, ObjectRequest, |t: Self::Item| {
//// TODO
//(
//t.bucket_id,
//t.object_id,
//Default::default(),
//Default::default(),
//None,
//)
//});

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
impl_message_decode!(ListObjectsRequestDecoder, ListObjectsRequest, |t: (String, u32, _)| {
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
impl_sized_message_encode!(ListObjectsRequestEncoder, ListObjectsRequest, |t: Self::Item| {
    (t.bucket_id, t.segment as u32)
});

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
