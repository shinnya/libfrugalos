//! server

use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Fields, MaybeDefault};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder};
use std::net::IpAddr;
use std::str::FromStr;

use entity::server::{Server, ServerSummary};

/// Decoder for `ServerSummary`.
#[derive(Debug, Default)]
pub struct ServerSummaryDecoder {
    inner: MessageDecoder<MaybeDefault<FieldDecoder<F1, StringDecoder>>>,
}

impl_message_decode!(ServerSummaryDecoder, ServerSummary, |t: _| {
    Ok(ServerSummary { id: t })
});

/// Encoder for `ServerSummary`.
#[derive(Debug, Default)]
pub struct ServerSummaryEncoder {
    inner: MessageEncoder<FieldEncoder<F1, StringEncoder>>,
}

impl_sized_message_encode!(ServerSummaryEncoder, ServerSummary, |item: Self::Item| {
    item.id
});

/// Decoder for `Server`.
#[derive(Debug, Default)]
pub struct ServerDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, StringDecoder>>,
            MaybeDefault<FieldDecoder<F4, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(ServerDecoder, Server, |t: (String, u32, String, u32)| {
    Ok(Server {
        id: t.0.clone(),
        seqno: t.1,
        host: track_any_err!(IpAddr::from_str(&t.2))?,
        port: t.3 as u16,
    })
});

/// Encoder for `Server`.
#[derive(Debug, Default)]
pub struct ServerEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, StringEncoder>,
            FieldEncoder<F4, Uint32Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(ServerEncoder, Server, |item: Self::Item| {
    (item.id, item.seqno, item.host.to_string(), item.port as u32)
});
