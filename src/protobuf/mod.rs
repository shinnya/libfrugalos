//! Encoders and decoders of Protocol Buffers.
//!
use protobuf_codec::field::Fields;
use protobuf_codec::message::{MessageDecoder, MessageEncoder};

pub mod consistency;
pub mod deadline;
pub mod entity;
pub mod expect;
pub mod schema;

/// Decoder for `()`.
#[derive(Debug, Default)]
pub struct UnitDecoder {
    inner: MessageDecoder<Fields<()>>,
}
impl_message_decode!(UnitDecoder, (), |t: _| Ok(t));

/// Encoder for `()`.
#[derive(Debug, Default)]
pub struct UnitEncoder {
    inner: MessageEncoder<Fields<()>>,
}
impl_sized_message_encode!(UnitEncoder, (), |item: Self::Item| item);
