use alloc::vec::Vec;

use crate::{utils, FromBytes, Result, ToBytes};
use bytes::{Buf, BufMut};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

impl FromBytes for BlockHeight {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = utils::Bytes::new(bytes);

        let h = reader.get_i64();

        Ok(BlockHeight(h))
    }
}

impl ToBytes for BlockHeight {
    type Bytes = Vec<u8>;

    fn to_bytes(&self) -> Result<Self::Bytes> {
        let mut bm = Vec::with_capacity(8);

        bm.put_i64(self.0);

        Ok(bm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_height() {
        let block_height = BlockHeight(0x12345678);

        let bytes = block_height.to_bytes().unwrap();

        let height = BlockHeight::from_bytes(&bytes).unwrap();

        assert_eq!(height, block_height);
    }
}
