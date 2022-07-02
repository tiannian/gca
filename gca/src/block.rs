use bytes::{Buf, BufMut};
use digest::{consts::U32, Digest};

use crate::{
    utils, BlockHash, BlockHeight, BytesSize, Error, FromBytes, IntoBytes, MerkleHash, NodeAddress,
    Result, Timestamp, ToBytes,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub height: BlockHeight,
    pub time: Option<Timestamp>,
    pub parent_hash: BlockHash,
    pub txs_hash: MerkleHash,
    pub app_hash: MerkleHash,
    pub proposer: NodeAddress,
}

impl BytesSize for BlockHeader {
    fn bytes_size() -> usize {
        8 + 8 + 32 + 32 + 20
    }
}

impl FromBytes for BlockHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let s = Self::bytes_size();

        if bytes.len() < s {
            return Err(Error::BytesSizeError(s, bytes.len()));
        }

        let mut bytes = utils::Bytes::new(bytes);

        let height = BlockHeight(bytes.get_i64());

        let time = Timestamp(bytes.get_i64());

        let mut parent_hash = BlockHash::default();
        bytes.copy_to_slice(parent_hash.as_mut());

        let mut app_hash = MerkleHash::default();
        bytes.copy_to_slice(app_hash.as_mut());

        let mut txs_hash = MerkleHash::default();
        bytes.copy_to_slice(txs_hash.as_mut());

        let mut proposer = NodeAddress::default();
        bytes.copy_to_slice(proposer.as_mut());

        Ok(Self {
            height,
            time: Some(time),
            parent_hash,
            app_hash,
            txs_hash,
            proposer,
        })
    }
}

impl ToBytes for BlockHeader {
    fn to_bytes(&self, buf: &mut impl BufMut) -> Result<()> {
        buf.put_i64(self.height.0);
        if let Some(t) = &self.time {
            buf.put_i64(t.0);
        } else {
            buf.put_i64(0);
        }
        buf.put_slice(self.parent_hash.as_ref());
        buf.put_slice(self.app_hash.as_ref());
        buf.put_slice(self.txs_hash.as_ref());
        buf.put_slice(self.proposer.as_ref());

        Ok(())
    }
}

impl IntoBytes for BlockHeader {}

#[derive(Debug, Default, Clone)]
pub struct Block {
    pub hash: BlockHash,
    pub header: BlockHeader,
}

impl Block {
    pub fn new<D: Digest<OutputSize = U32>>(header: BlockHeader) -> Result<Self> {
        let bytes = header.into_bytes()?;

        let hash = D::digest(bytes);

        let hash = BlockHash::from_slice(&hash);

        Ok(Self { hash, header })
    }
}

impl BytesSize for Block {
    fn bytes_size() -> usize {
        32 + 8 + 8 + 32 + 32 + 20
    }
}

impl FromBytes for Block {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let s = Self::bytes_size();

        if bytes.len() < s {
            return Err(Error::BytesSizeError(s, bytes.len()));
        }

        let mut bytes = utils::Bytes::new(bytes);

        let mut hash = BlockHash::default();
        bytes.copy_to_slice(hash.as_mut());

        let header = BlockHeader::from_bytes(bytes.chunk())?;

        Ok(Self { hash, header })
    }
}

impl ToBytes for Block {
    fn to_bytes(&self, buf: &mut impl BufMut) -> Result<()> {
        buf.put_slice(self.hash.as_ref());

        self.header.to_bytes(buf)?;

        Ok(())
    }
}

impl IntoBytes for Block {}

#[cfg(test)]
mod tests {
    use primitive_types::{H160, H256};
    use sha3::Sha3_256;

    use super::*;

    fn build_block_header() -> BlockHeader {
        let bytes = [0x0u8, 0x0];
        let hi = Sha3_256::digest(bytes);
        let hash = H256::from_slice(&hi);

        BlockHeader {
            height: BlockHeight(0x12345678),
            time: Some(Timestamp(0x12345678)),
            parent_hash: BlockHash(hash.clone()),
            txs_hash: MerkleHash(hash.clone()),
            app_hash: MerkleHash(hash.clone()),
            proposer: NodeAddress(H160::from(hash)),
        }
    }

    #[test]
    fn test_block_header() {
        let header = build_block_header();

        let by = header.into_bytes().unwrap();

        let header1 = BlockHeader::from_bytes(&by).unwrap();

        assert_eq!(header, header1);
    }

    #[test]
    fn test_block() {
        let header = build_block_header();
        let block = Block::new::<Sha3_256>(header).unwrap();

        let by = block.into_bytes().unwrap();

        let block1 = Block::from_bytes(&by).unwrap();

        assert_eq!(block.hash, block1.hash);
        assert_eq!(block.header, block1.header);
    }
}
