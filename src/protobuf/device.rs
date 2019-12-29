//! Device

use bytecodec::combinator::PreEncode;
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
use std::collections::BTreeSet;
use std::path::PathBuf;

use entity::device::{
    Device, DeviceKind, DeviceSummary, FileDevice, MemoryDevice, SegmentAllocationPolicy,
    VirtualDevice, Weight,
};
//use ErrorKind;
use Result;

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
    let kind = match t.2 {
        0 => DeviceKind::Virtual,
        1 => DeviceKind::Memory,
        2 => DeviceKind::File,
        _n => DeviceKind::Memory,
        // TODO
        //n => track_panic!(ErrorKind::InvalidInput, "Unknown device kind: {}", n),
    };
    Ok(DeviceSummary {
        id: t.0.clone(),
        server: t.1.clone(),
        kind,
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
    match x {
        0 => Ok(SegmentAllocationPolicy::ScatterIfPossible),
        1 => Ok(SegmentAllocationPolicy::Scatter),
        2 => Ok(SegmentAllocationPolicy::Neutral),
        3 => Ok(SegmentAllocationPolicy::Gather),
        4 => Ok(SegmentAllocationPolicy::AsEvenAsPossible),
        // TODO
        _n => Ok(SegmentAllocationPolicy::ScatterIfPossible),
    }
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
    // TODO
    //let policy = track!(decode_segment_allocation_policy(t.4))?;
    let policy = decode_segment_allocation_policy(t.4).unwrap();
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

impl_sized_message_encode!(FileDeviceEncoder, FileDevice, |item: Self::Item| {
    // TODO
    let filepath = item.filepath.into_os_string().into_string().unwrap();
    (
        item.id,
        item.seqno,
        item.weight,
        item.server,
        item.capacity,
        filepath,
    )
});
