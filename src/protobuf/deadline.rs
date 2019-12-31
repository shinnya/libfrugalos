//! Deadline

use bytecodec::Result;
use protobuf_codec::scalar::{Uint64Decoder, Uint64Encoder};
use std::time::Duration;

/// Decoder for `Deadline`.
// 互換性に注意: ここではミリ秒で扱っているが frugalos では秒で扱っている
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L98
pub type DeadlineDecoder = Uint64Decoder;

/// Encoder for `Deadline`.
// 互換性に注意: ここではミリ秒で扱っているが frugalos では秒で扱っている
// https://github.com/frugalos/frugalos/blob/346b56c23a0055f160da385668ce163ee8ff6e60/frugalos_mds/src/protobuf.rs#L109
pub type DeadlineEncoder = Uint64Encoder;

/// Decodes `Deadline`.
pub fn decode_deadline(millis: u64) -> Result<Duration> {
    Ok(Duration::from_millis(millis))
}

/// Encodes `Deadline`.
pub fn encode_deadline(deadline: Duration) -> u64 {
    // u64 におさめるため
    deadline.as_millis() as u64
}
