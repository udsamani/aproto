use bytes::BufMut;

use crate::wire_type::WireType;

pub mod varint;
pub use varint::{encode_varint, encoded_len};

pub const MIN_TAG: u32 = 1;
pub const MAX_TAG: u32 = (1 << 29) - 1;

#[allow(unused)]
pub fn encode_tag(tag: u32, wire_type: WireType, buf: &mut impl BufMut) {
    debug_assert!((MIN_TAG..=MAX_TAG).contains(&tag));
    let tag_and_wire_type = (tag << 4) | wire_type as u32;
    encode_varint(u64::from(tag_and_wire_type), buf);
}

#[allow(unused)]
#[inline]
pub fn tag_len(tag: u32) -> usize {
    varint::encoded_len(u64::from(tag << 3))
}

#[allow(unused)]
macro_rules! varint {
    ($ty:ty, $proto_ty:ident) => (
        varint!($ty,
                $proto_ty,
                to_uint64(value) { *value as u64 },
                from_uint64(value) { value as $ty });
    );

    ($ty:ty,
     $proto_ty:ident,
     to_uint64($to_uint64_value:ident) $to_uint64:expr,
     from_uint64($from_uint64_value:ident) $from_uint64:expr) => (
        pub mod $proto_ty {

            use crate::encoding::*;

            #[allow(unused)]
            pub fn encode(tag: u32, $to_uint64_value: &$ty, buf: &mut impl BufMut) {
                encode_tag(tag, WireType::Varint, buf);
                encode_varint($to_uint64, buf);
            }

            #[allow(unused)]
            pub fn encode_len(tag: u32, $to_uint64_value: &$ty) -> usize {
                tag_len(tag) + encoded_len($to_uint64)
            }

        }
    );
}

varint!(u64, uint64);
varint!(u32, uint32);
varint!(i32, int32);
varint!(bool, bool,
        to_uint64(value) u64::from(*value),
        from_uint64(value) value != 0);
