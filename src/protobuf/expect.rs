//! Expect

use bytecodec::combinator::PreEncode;
use protobuf_codec::field::branch::Branch4;
use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MessageFieldDecoder, MessageFieldEncoder, Oneof,
    PackedFieldDecoder, PackedFieldEncoder, Repeated,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{Uint32Decoder, Uint32Encoder, Uint64Decoder, Uint64Encoder};

use entity::object::ObjectVersion;
use expect::Expect;
use protobuf::object::{ObjectVersionsDecoder, ObjectVersionsEncoder};

/// Decoder for `Expect`.
#[derive(Debug, Default)]
pub struct ExpectDecoder {
    inner: MessageDecoder<
        Oneof<(
            FieldDecoder<F1, Uint32Decoder>,
            FieldDecoder<F2, Uint32Decoder>,
            MessageFieldDecoder<F3, ObjectVersionsDecoder>,
            MessageFieldDecoder<F4, ObjectVersionsDecoder>,
        )>,
    >,
}
impl_message_decode!(ExpectDecoder, Expect, |t: Branch4<
    u32,
    u32,
    Vec<u64>,
    Vec<u64>,
>| {
    Ok(match t {
        Branch4::A(_) => Expect::Any,
        Branch4::B(_) => Expect::None,
        Branch4::C(versions) => Expect::IfMatch(versions.into_iter().map(ObjectVersion).collect()),
        Branch4::D(versions) => {
            Expect::IfNoneMatch(versions.into_iter().map(ObjectVersion).collect())
        }
    })
});

/// Encoder for `Expect`.
#[derive(Debug, Default)]
pub struct ExpectEncoder {
    inner: MessageEncoder<
        Oneof<(
            FieldEncoder<F1, Uint32Encoder>,
            FieldEncoder<F2, Uint32Encoder>,
            MessageFieldEncoder<F3, PreEncode<ObjectVersionsEncoder>>,
            MessageFieldEncoder<F4, PreEncode<ObjectVersionsEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(ExpectEncoder, Expect, |item: Self::Item| {
    match item {
        Expect::Any => Branch4::A(0),
        Expect::None => Branch4::B(1),
        Expect::IfMatch(versions) => Branch4::C(versions.into_iter().map(|v| v.0).collect()),
        Expect::IfNoneMatch(versions) => Branch4::D(versions.into_iter().map(|v| v.0).collect()),
    }
});
