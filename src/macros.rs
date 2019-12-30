macro_rules! impl_message_decode {
    ($decoder:ty, $item:ty, $map:expr) => {
        impl ::bytecodec::Decode for $decoder {
            type Item = $item;

            fn decode(&mut self, buf: &[u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
                track!(self.inner.decode(buf, eos))
            }

            fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
                let item = track!(self.inner.finish_decoding())?;
                $map(item)
            }

            fn is_idle(&self) -> bool {
                self.inner.is_idle()
            }

            fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
                self.inner.requiring_bytes()
            }
        }
        impl ::protobuf_codec::message::MessageDecode for $decoder {}
    };
}

macro_rules! impl_message_encode {
    ($encoder:ty, $item:ty, $map:expr) => {
        impl ::bytecodec::Encode for $encoder {
            type Item = $item;

            fn encode(
                &mut self,
                buf: &mut [u8],
                eos: ::bytecodec::Eos,
            ) -> ::bytecodec::Result<usize> {
                track!(self.inner.encode(buf, eos))
            }

            fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
                track!(self.inner.start_encoding($map(item)))
            }

            fn is_idle(&self) -> bool {
                self.inner.is_idle()
            }

            fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
                self.inner.requiring_bytes()
            }
        }
        impl ::protobuf_codec::message::MessageEncode for $encoder {}
    };
}

macro_rules! impl_sized_message_encode {
    ($encoder:ty, $item:ty, $map:expr) => {
        impl_message_encode!($encoder, $item, $map);
        impl ::bytecodec::SizedEncode for $encoder {
            fn exact_requiring_bytes(&self) -> u64 {
                self.inner.exact_requiring_bytes()
            }
        }
    };
}

// TODO remove
macro_rules! impl_newtype_decode {
    ($decoder:ty, $item:ty) => {
        impl ::bytecodec::Decode for $decoder {
            type Item = $item;

            fn decode(&mut self, buf: &[u8], eos: Eos) -> ::bytecodec::Result<usize> {
                track!(self.0.decode(buf, eos))
            }

            fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
                let item = track!(self.0.finish_decoding())?;
                $item(item)
            }

            fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
                self.0.requiring_bytes()
            }

            fn is_idle(&self) -> bool {
                self.0.is_idle()
            }
        }
        impl ::protobuf_codec::value::ValueDecode for $decoder {
            fn wire_type(&self) -> ::protobuf_codec::wire::WireType {
                use protobuf_codec::value::ValueDecode;
                self.0.wire_type()
            }
        }
    };
}

// TODO remove
macro_rules! impl_newtype_encode {
    ($encoder:ty, $item:ty) => {
        impl ::bytecodec::Encode for $encoder {
            type Item = $item;

            fn encode(&mut self, buf: &mut [u8], eos: Eos) -> ::bytecodec::Result<usize> {
                track!(self.0.encode(buf, eos))
            }

            fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
                track!(self.0.start_encoding(item.0))
            }

            fn is_idle(&self) -> bool {
                self.0.is_idle()
            }

            fn requiring_bytes(&self) -> ByteCount {
                self.0.requiring_bytes()
            }
        }
        impl ::bytecodec::SizedEncode for $encoder {
            fn exact_requiring_bytes(&self) -> u64 {
                self.0.exact_requiring_bytes()
            }
        }
        impl ::protobuf_codec::value::ValueEncode for $encoder {
            fn wire_type(&self) -> ::protobuf_codec::wire::WireType {
                use protobuf_codec::value::ValueEncode;
                self.0.wire_type()
            }
        }
    };
}
