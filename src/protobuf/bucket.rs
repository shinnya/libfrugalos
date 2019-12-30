//! aaa

use bytecodec::ErrorKind;
use protobuf_codec::field::branch::Branch3;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5, F6};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder};

use entity::bucket::{
    Bucket, BucketKind, BucketSummary, DispersedBucket, MetadataBucket, ReplicatedBucket,
};

/// Decoder for `BucketSummary`.
#[derive(Debug, Default)]
pub struct BucketSummaryDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, StringDecoder>>,
        )>,
    >,
}

impl_message_decode!(BucketSummaryDecoder, BucketSummary, |t: (
    String,
    u32,
    String,
)| {
    Ok(BucketSummary {
        id: t.0.clone(),
        kind: match t.1 {
            0 => BucketKind::Metadata,
            1 => BucketKind::Replicated,
            2 => BucketKind::Dispersed,
            n => track_panic!(ErrorKind::InvalidInput, "Unknown bucket kind: {}", n),
        },
        device: t.2.clone(),
    })
});

/// Encoder for `BucketSummary`.
#[derive(Debug, Default)]
pub struct BucketSummaryEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, StringEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(BucketSummaryEncoder, BucketSummary, |item: Self::Item| {
    let kind = match item.kind {
        BucketKind::Metadata => 0,
        BucketKind::Replicated => 1,
        BucketKind::Dispersed => 2,
    };
    (item.id, kind, item.device)
});

/// Decoder for `Bucket`.
#[derive(Debug, Default)]
pub struct BucketDecoder {
    inner: MessageDecoder<
        Oneof<(
            MessageFieldDecoder<F1, MetadataBucketDecoder>,
            MessageFieldDecoder<F2, ReplicatedBucketDecoder>,
            MessageFieldDecoder<F3, DispersedBucketDecoder>,
        )>,
    >,
}

impl_message_decode!(BucketDecoder, Bucket, |t: _| {
    Ok(match t {
        Branch3::A(bucket) => Bucket::Metadata(bucket),
        Branch3::B(bucket) => Bucket::Replicated(bucket),
        Branch3::C(bucket) => Bucket::Dispersed(bucket),
    })
});

/// Encoder for `Bucket`.
#[derive(Debug, Default)]
pub struct BucketEncoder {
    inner: MessageEncoder<
        Oneof<(
            MessageFieldEncoder<F1, MetadataBucketEncoder>,
            MessageFieldEncoder<F2, ReplicatedBucketEncoder>,
            MessageFieldEncoder<F3, DispersedBucketEncoder>,
        )>,
    >,
}

impl_sized_message_encode!(BucketEncoder, Bucket, |item: Self::Item| match item {
    Bucket::Metadata(bucket) => Branch3::A(bucket),
    Bucket::Replicated(bucket) => Branch3::B(bucket),
    Bucket::Dispersed(bucket) => Branch3::C(bucket),
});

/// Decoder for `MetadataBucket`.
#[derive(Debug, Default)]
pub struct MetadataBucketDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, StringDecoder>>,
            MaybeDefault<FieldDecoder<F4, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F5, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(MetadataBucketDecoder, MetadataBucket, |t: (
    String,
    u32,
    String,
    u32,
    u32
)| {
    Ok(MetadataBucket {
        id: t.0.clone(),
        seqno: t.1,
        device: t.2.clone(),
        segment_count: t.3,
        tolerable_faults: t.4,
    })
});

/// Encoder for `MetadataBucket`.
#[derive(Debug, Default)]
pub struct MetadataBucketEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, StringEncoder>,
            FieldEncoder<F4, Uint32Encoder>,
            FieldEncoder<F5, Uint32Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(MetadataBucketEncoder, MetadataBucket, |item: Self::Item| {
    (
        item.id,
        item.seqno,
        item.device,
        item.segment_count,
        item.tolerable_faults,
    )
});

/// Decoder for `ReplicatedBucket`.
#[derive(Debug, Default)]
pub struct ReplicatedBucketDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, StringDecoder>>,
            MaybeDefault<FieldDecoder<F4, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F5, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(ReplicatedBucketDecoder, ReplicatedBucket, |t: (
    String,
    u32,
    String,
    u32,
    u32
)| {
    Ok(ReplicatedBucket {
        id: t.0.clone(),
        seqno: t.1,
        device: t.2.clone(),
        segment_count: t.3,
        tolerable_faults: t.4,
    })
});

/// Encoder for `ReplicatedBucket`.
#[derive(Debug, Default)]
pub struct ReplicatedBucketEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, StringEncoder>,
            FieldEncoder<F4, Uint32Encoder>,
            FieldEncoder<F5, Uint32Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(
    ReplicatedBucketEncoder,
    ReplicatedBucket,
    |item: Self::Item| {
        (
            item.id,
            item.seqno,
            item.device,
            item.segment_count,
            item.tolerable_faults,
        )
    }
);

/// Decoder for `DispersedBucket`.
#[derive(Debug, Default)]
pub struct DispersedBucketDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F3, StringDecoder>>,
            MaybeDefault<FieldDecoder<F4, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F5, Uint32Decoder>>,
            MaybeDefault<FieldDecoder<F6, Uint32Decoder>>,
        )>,
    >,
}

impl_message_decode!(DispersedBucketDecoder, DispersedBucket, |t: (
    String,
    u32,
    String,
    u32,
    u32,
    u32
)| {
    Ok(DispersedBucket {
        id: t.0.clone(),
        seqno: t.1,
        device: t.2.clone(),
        segment_count: t.3,
        tolerable_faults: t.4,
        data_fragment_count: t.5,
    })
});

/// Encoder for `DispersedBucket`.
#[derive(Debug, Default)]
pub struct DispersedBucketEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, Uint32Encoder>,
            FieldEncoder<F3, StringEncoder>,
            FieldEncoder<F4, Uint32Encoder>,
            FieldEncoder<F5, Uint32Encoder>,
            FieldEncoder<F6, Uint32Encoder>,
        )>,
    >,
}

impl_sized_message_encode!(
    DispersedBucketEncoder,
    DispersedBucket,
    |item: Self::Item| {
        (
            item.id,
            item.seqno,
            item.device,
            item.segment_count,
            item.tolerable_faults,
            item.data_fragment_count,
        )
    }
);
