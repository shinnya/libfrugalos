//! Encoders and decoders of Protocol Buffers.

use bytecodec::combinator::PreEncode;
use bytecodec::SizedEncode;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2};
use protobuf_codec::field::{MessageFieldDecoder, MessageFieldEncoder, Oneof};
use protobuf_codec::message::{MessageDecode, MessageDecoder, MessageEncode, MessageEncoder};
use trackable::error::ErrorKindExt;

use protobuf::error::{ErrorDecoder, ErrorEncoder};
use {ErrorKind, Result};

pub mod consistency;
pub mod deadline;
pub mod entity;
pub mod error;
pub mod expect;
pub mod schema;

/// Decoder for `Result`.
#[derive(Debug, Default)]
pub struct ResultDecoder<D: MessageDecode> {
    inner: MessageDecoder<
        Oneof<(
            MessageFieldDecoder<F1, D>,
            MessageFieldDecoder<F2, ErrorDecoder>,
        )>,
    >,
}
impl<D: MessageDecode> ::bytecodec::Decode for ResultDecoder<D> {
    type Item = Result<D::Item>;

    fn decode(&mut self, buf: &[u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
        match track!(self.inner.finish_decoding())? {
            Branch2::A(value) => Ok(Ok(value)),
            // TODO InvalidInput 再検討
            Branch2::B(e) => Ok(track!(Err(ErrorKind::InvalidInput.takes_over(e).into()))),
        }
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<D: MessageDecode> ::protobuf_codec::message::MessageDecode for ResultDecoder<D> {}

/// Encoder for `Result`.
#[derive(Debug, Default)]
pub struct ResultEncoder<E: MessageEncode + SizedEncode> {
    inner: MessageEncoder<
        Oneof<(
            MessageFieldEncoder<F1, E>,
            MessageFieldEncoder<F2, PreEncode<ErrorEncoder>>,
        )>,
    >,
}
impl<E: MessageEncode + SizedEncode> ::bytecodec::Encode for ResultEncoder<E> {
    type Item = Result<E::Item>;

    fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
        let item = match item {
            Ok(x) => Branch2::A(x),
            Err(e) => {
                // TODO InvalidInput 再検討
                Branch2::B(ErrorKind::InvalidInput.takes_over(e))
            }
        };
        track!(self.inner.start_encoding(item))
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }
}
impl<E: MessageEncode + SizedEncode> ::protobuf_codec::message::MessageEncode for ResultEncoder<E> {}
impl<E: MessageEncode + SizedEncode> ::bytecodec::SizedEncode for ResultEncoder<E> {
    fn exact_requiring_bytes(&self) -> u64 {
        self.inner.exact_requiring_bytes()
    }
}
