use crate::{utils, FromBytes, IntoBytes, Result, ToBytes, BytesSize, Error};
use bytes::{Buf, BufMut};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

impl FromBytes for BlockHeight {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let s = Self::bytes_size();

        if bytes.len() < s {
            return Err(Error::BytesSizeError(s, bytes.len()));
        }

        let mut reader = utils::Bytes::new(bytes);

        let h = reader.get_i64();

        Ok(BlockHeight(h))
    }
}

impl BytesSize for BlockHeight {
    fn bytes_size() -> usize {
        8
    }
}

impl ToBytes for BlockHeight {
    fn to_bytes(&self, buf: &mut impl BufMut) -> Result<()> {
        buf.put_i64(self.0);
        Ok(())
    }
}

impl IntoBytes for BlockHeight {}

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
