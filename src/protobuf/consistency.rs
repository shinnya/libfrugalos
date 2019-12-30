//! aaa

use protobuf_codec::field::branch::Branch4;
use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Oneof};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{Uint32Decoder, Uint32Encoder};

use consistency::ReadConsistency;

/// Decoder for `ReadConsistency`.
#[derive(Debug, Default)]
pub struct ReadConsistencyDecoder {
    inner: MessageDecoder<
        Oneof<(
            FieldDecoder<F1, Uint32Decoder>,
            FieldDecoder<F2, Uint32Decoder>,
            FieldDecoder<F3, Uint32Decoder>,
            FieldDecoder<F4, Uint32Decoder>,
        )>,
    >,
}
impl_message_decode!(ReadConsistencyDecoder, ReadConsistency, |t: _| {
    Ok(match t {
        Branch4::A(_) => ReadConsistency::Consistent,
        Branch4::B(_) => ReadConsistency::Stale,
        Branch4::C(_) => ReadConsistency::Quorum,
        Branch4::D(n) => ReadConsistency::Subset(n as usize),
    })
});

/// Encoder for `ReadConsistency`.
#[derive(Debug, Default)]
pub struct ReadConsistencyEncoder {
    inner: MessageEncoder<
        Oneof<(
            FieldEncoder<F1, Uint32Encoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, Uint32Encoder>,
            FieldEncoder<F4, Uint32Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(ReadConsistencyEncoder, ReadConsistency, |item: Self::Item| {
    // TODO enum に対応する値が必要ない場合どうするか？を検討
    match item {
        ReadConsistency::Consistent => Branch4::A(0),
        ReadConsistency::Stale => Branch4::B(0),
        ReadConsistency::Quorum => Branch4::C(0),
        ReadConsistency::Subset(n) => Branch4::D(n as u32),
    }
});
