//! Device

use bytecodec::combinator::PreEncode;
use bytecodec::{ErrorKind, Result};
use protobuf_codec::field::branch::Branch3;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5, F6};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof, Optional, Repeated,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    DoubleDecoder, DoubleEncoder, StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder,
    Uint64Decoder, Uint64Encoder,
};
use std::borrow::ToOwned;
use std::collections::BTreeSet;
use std::path::PathBuf;
use trackable::error::ErrorKindExt;

use entity::device::{
    Device, DeviceKind, DeviceSummary, FileDevice, MemoryDevice, SegmentAllocationPolicy,
    VirtualDevice, Weight,
};

/// Decoder for `DeviceSummary`.
#[derive(Debug, Default)]
pub struct DeviceSummaryDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            Optional<FieldDecoder<F2, StringDecoder>>,
            MaybeDefault<FieldDecoder<F3, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(DeviceSummaryDecoder, DeviceSummary, |t: (
    String,
    Option<String>,
    u32,
)| {
    Ok(DeviceSummary {
        id: t.0.clone(),
        server: t.1.clone(),
        kind: match t.2 {
            0 => DeviceKind::Virtual,
            1 => DeviceKind::Memory,
            2 => DeviceKind::File,
            n => track_panic!(ErrorKind::InvalidInput, "Unknown device kind: {}", n),
        },
    })
});

/// Encoder for `DeviceSummary`.
#[derive(Debug, Default)]
pub struct DeviceSummaryEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            Optional<FieldEncoder<F2, StringEncoder>>,
            FieldEncoder<F3, Uint32Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(DeviceSummaryEncoder, DeviceSummary, |item: Self::Item| {
    let kind = match item.kind {
        DeviceKind::Virtual => 0,
        DeviceKind::Memory => 1,
        DeviceKind::File => 2,
    };
    (item.id, item.server, kind)
});

/// Decoder for `Device`.
#[derive(Debug, Default)]
pub struct DeviceDecoder {
    inner: MessageDecoder<
        Oneof<(
            MessageFieldDecoder<F1, VirtualDeviceDecoder>,
            MessageFieldDecoder<F2, MemoryDeviceDecoder>,
            MessageFieldDecoder<F3, FileDeviceDecoder>,
        )>,
    >,
}

impl_message_decode!(DeviceDecoder, Device, |t: _| {
    Ok(match t {
        Branch3::A(device) => Device::Virtual(device),
        Branch3::B(device) => Device::Memory(device),
        Branch3::C(device) => Device::File(device),
    })
});

/// Encoder for `Device`.
#[derive(Debug, Default)]
pub struct DeviceEncoder {
    inner: MessageEncoder<
        Oneof<(
            MessageFieldEncoder<F1, PreEncode<VirtualDeviceEncoder>>,
            MessageFieldEncoder<F2, MemoryDeviceEncoder>,
            MessageFieldEncoder<F3, FileDeviceEncoder>,
        )>,
    >,
}

impl_message_encode!(DeviceEncoder, Device, |item: Self::Item| match item {
    Device::Virtual(device) => Branch3::A(device),
    Device::Memory(device) => Branch3::B(device),
    Device::File(device) => Branch3::C(device),
});

/// Decoder for `Weight`.
#[derive(Debug, Default)]
pub struct WeightDecoder {
    inner: MessageDecoder<
        Oneof<(
            FieldDecoder<F1, Uint32Decoder>,
            FieldDecoder<F2, Uint64Decoder>,
            FieldDecoder<F3, DoubleDecoder>,
        )>,
    >,
}

impl_message_decode!(WeightDecoder, Weight, |t: _| {
    Ok(match t {
        Branch3::A(_) => Weight::Auto,
        Branch3::B(n) => Weight::Absolute(n),
        Branch3::C(n) => Weight::Relative(n),
    })
});

/// Encoder for `Weight`.
#[derive(Debug, Default)]
pub struct WeightEncoder {
    inner: MessageEncoder<
        Oneof<(
            FieldEncoder<F1, Uint32Encoder>,
            FieldEncoder<F2, Uint64Encoder>,
            FieldEncoder<F3, DoubleEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(WeightEncoder, Weight, |item: Self::Item| match item {
    Weight::Auto => Branch3::A(0),
    Weight::Absolute(n) => Branch3::B(n),
    Weight::Relative(n) => Branch3::C(n),
});

fn decode_segment_allocation_policy(x: u32) -> Result<SegmentAllocationPolicy> {
    Ok(match x {
        0 => SegmentAllocationPolicy::ScatterIfPossible,
        1 => SegmentAllocationPolicy::Scatter,
        2 => SegmentAllocationPolicy::Neutral,
        3 => SegmentAllocationPolicy::Gather,
        4 => SegmentAllocationPolicy::AsEvenAsPossible,
        n => track_panic!(
            ErrorKind::InvalidInput,
            "Unknown SegmentAllocationPolicy: {}",
            n
        ),
    })
}

fn encode_segment_allocation_policy(policy: SegmentAllocationPolicy) -> u32 {
    match policy {
        SegmentAllocationPolicy::ScatterIfPossible => 0,
        SegmentAllocationPolicy::Scatter => 1,
        SegmentAllocationPolicy::Neutral => 2,
        SegmentAllocationPolicy::Gather => 3,
        SegmentAllocationPolicy::AsEvenAsPossible => 4,
    }
}

/// Decoder for `VirtualDevice`.
#[derive(Debug, Default)]
pub struct VirtualDeviceDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<MessageFieldDecoder<F3, WeightDecoder>>,
            Repeated<FieldDecoder<F4, StringDecoder>, Vec<String>>,
            MaybeDefault<FieldDecoder<F5, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(VirtualDeviceDecoder, VirtualDevice, |t: (
    String,
    u32,
    Weight,
    Vec<_>,
    u32,
)| {
    let policy = track!(decode_segment_allocation_policy(t.4))?;
    Ok(VirtualDevice {
        id: t.0.clone(),
        seqno: t.1,
        weight: t.2.clone(),
        children: t.3.into_iter().collect::<BTreeSet<_>>(),
        policy,
    })
});

/// Encoder for `VirtualDevice`.
#[derive(Debug, Default)]
pub struct VirtualDeviceEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            MessageFieldEncoder<F3, WeightEncoder>,
            Repeated<FieldEncoder<F4, StringEncoder>, Vec<String>>,
            FieldEncoder<F5, Uint32Encoder>,
        )>,
    >,
}

impl_message_encode!(VirtualDeviceEncoder, VirtualDevice, |item: Self::Item| {
    (
        item.id,
        item.seqno,
        item.weight,
        item.children.into_iter().collect::<Vec<_>>(),
        encode_segment_allocation_policy(item.policy),
    )
});

/// Decoder for `MemoryDevice`.
#[derive(Debug, Default)]
pub struct MemoryDeviceDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<MessageFieldDecoder<F3, WeightDecoder>>,
            MaybeDefault<FieldDecoder<F4, StringDecoder>>,
            MaybeDefault<FieldDecoder<F5, Uint64Decoder>>,
        )>,
    >,
}

impl_message_decode!(MemoryDeviceDecoder, MemoryDevice, |t: (
    String,
    u32,
    Weight,
    String,
    u64,
)| {
    Ok(MemoryDevice {
        id: t.0.clone(),
        seqno: t.1,
        weight: t.2.clone(),
        server: t.3.clone(),
        capacity: t.4,
    })
});

/// Encoder for `MemoryDevice`.
#[derive(Debug, Default)]
pub struct MemoryDeviceEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            MessageFieldEncoder<F3, WeightEncoder>,
            FieldEncoder<F4, StringEncoder>,
            FieldEncoder<F5, Uint64Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(MemoryDeviceEncoder, MemoryDevice, |item: Self::Item| {
    (item.id, item.seqno, item.weight, item.server, item.capacity)
});

/// Decoder for `FileDevice`.
#[derive(Debug, Default)]
pub struct FileDeviceDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<MessageFieldDecoder<F3, WeightDecoder>>,
            MaybeDefault<FieldDecoder<F4, StringDecoder>>,
            MaybeDefault<FieldDecoder<F5, Uint64Decoder>>,
            // パスは valid な UTF-8 に制限してしまう
            MaybeDefault<FieldDecoder<F6, StringDecoder>>,
        )>,
    >,
}

impl_message_decode!(FileDeviceDecoder, FileDevice, |t: (
    String,
    u32,
    Weight,
    String,
    u64,
    String,
)| {
    Ok(FileDevice {
        id: t.0.clone(),
        seqno: t.1,
        weight: t.2.clone(),
        server: t.3.clone(),
        capacity: t.4,
        filepath: PathBuf::from(t.5),
    })
});

/// Encoder for `FileDevice`.
#[derive(Debug, Default)]
pub struct FileDeviceEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            MessageFieldEncoder<F3, WeightEncoder>,
            FieldEncoder<F4, StringEncoder>,
            FieldEncoder<F5, Uint64Encoder>,
            FieldEncoder<F6, StringEncoder>,
        )>,
    >,
}
// `FileDeviceEncoder` はバリデーションが必要なのでマクロを使わずに実装する
impl ::bytecodec::Encode for FileDeviceEncoder {
    type Item = FileDevice;

    fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
        track!(self.inner.encode(buf, eos))
    }

    fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
        let filepath = track!(item
            .filepath
            .to_str()
            .map(ToOwned::to_owned)
            .ok_or(ErrorKind::InvalidInput.cause("filepath is not a valid UTF-8")))?;
        track!(self.inner.start_encoding((
            item.id,
            item.seqno,
            item.weight,
            item.server,
            item.capacity,
            filepath,
        )))
    }

    fn is_idle(&self) -> bool {
        self.inner.is_idle()
    }

    fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
        self.inner.requiring_bytes()
    }
}
impl ::protobuf_codec::message::MessageEncode for FileDeviceEncoder {}
impl ::bytecodec::SizedEncode for FileDeviceEncoder {
    fn exact_requiring_bytes(&self) -> u64 {
        self.inner.exact_requiring_bytes()
    }
}
