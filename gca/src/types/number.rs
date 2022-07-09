use crate::{utils, BytesSize, Error, FromBytes, IntoBytes, Result, ToBytes};
use bytes::{Buf, BufMut};

macro_rules! impl_bytes {
    ($ty:ty, $get:ident, $put:ident, $length:expr) => {
        impl FromBytes for $ty {
            fn from_bytes(bytes: &[u8]) -> Result<Self> {
                let s = Self::bytes_size();

                if bytes.len() < s {
                    return Err(Error::BytesSizeError(s, bytes.len()));
                }

                let mut reader = utils::Bytes::new(bytes);

                let h = reader.$get();

                Ok(Self(h))
            }
        }

        impl BytesSize for $ty {
            fn bytes_size() -> usize {
                $length
            }
        }

        impl ToBytes for $ty {
            fn to_bytes(&self, buf: &mut impl BufMut) -> Result<()> {
                buf.$put(self.0);
                Ok(())
            }
        }

        impl IntoBytes for $ty {}
    };
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);
impl_bytes!(BlockHeight, get_i64, put_i64, 8);

#[derive(Debug, Default, Clone)]
pub struct MemoOperation(pub u32);
impl_bytes!(MemoOperation, get_u32, put_u32, 4);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputOperation(pub u32);
impl_bytes!(OutputOperation, get_u32, put_u32, 4);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Amount(pub u64);
impl_bytes!(Amount, get_u64, put_u64, 8);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Timestamp(pub i64);
impl_bytes!(Timestamp, get_i64, put_i64, 8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_height() {
        let block_height = BlockHeight(0x12345678);

        let bytes = block_height.into_bytes().unwrap();

        let height = BlockHeight::from_bytes(&bytes).unwrap();

        assert_eq!(height, block_height);
    }
}
