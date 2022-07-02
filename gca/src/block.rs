use alloc::vec::Vec;
use bytes::{Buf, BufMut};
use digest::{consts::U32, Digest};

use crate::{
    utils, BlockHash, BlockHeight, FromBytes, MerkleHash, NodeAddress, Result, Timestamp, ToBytes,
};

#[derive(Debug, Default, Clone)]
pub struct BlockHeader {
    pub height: BlockHeight,
    pub time: Option<Timestamp>,
    pub parent_hash: BlockHash,
    pub txs_hash: MerkleHash,
    pub app_hash: MerkleHash,
    pub proposer: NodeAddress,
}

impl BlockHeader {}

impl FromBytes for BlockHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
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
    type Bytes = Vec<u8>;

    fn to_bytes(&self) -> Result<Self::Bytes> {
        const fn bytes_len() -> usize {
            8 + 8 + 32 + 32 + 20
        }

        let capacity = bytes_len();

        let mut bytes = Vec::with_capacity(capacity);

        bytes.put_i64(self.height.0);
        if let Some(t) = &self.time {
            bytes.put_i64(t.0);
        } else {
            bytes.put_i64(0);
        }
        bytes.put_slice(self.parent_hash.as_ref());
        bytes.put_slice(self.app_hash.as_ref());
        bytes.put_slice(self.txs_hash.as_ref());
        bytes.put_slice(self.proposer.as_ref());

        Ok(bytes)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Block {
    pub hash: BlockHash,
    pub header: BlockHeader,
}

impl Block {
    // pub fn new<D: Digest<OutputSize = U32>>(header: BlockHeader) -> Self {}
}

impl FromBytes for Block {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut bytes = utils::Bytes::new(bytes);

        let mut hash = BlockHash::default();
        bytes.copy_to_slice(hash.as_mut());

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
            hash,
            header: BlockHeader {
                height,
                time: Some(time),
                parent_hash,
                app_hash,
                txs_hash,
                proposer,
            },
        })
    }
}

const fn bytes_len() -> usize {
    32 + 8 + 8 + 32 + 32 + 20
}

impl ToBytes for Block {
    type Bytes = Vec<u8>;

    fn to_bytes(&self) -> Result<Self::Bytes> {
        let capacity = bytes_len();

        let mut bytes = Vec::with_capacity(capacity);

        bytes.put_slice(self.hash.as_ref());
        bytes.put_i64(self.header.height.0);
        if let Some(t) = &self.header.time {
            bytes.put_i64(t.0);
        } else {
            bytes.put_i64(0);
        }
        bytes.put_slice(self.header.app_hash.as_ref());
        bytes.put_slice(self.header.txs_hash.as_ref());
        bytes.put_slice(self.header.proposer.as_ref());

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use primitive_types::{H160, H256};
    use sha3::Sha3_256;

    use super::*;

    #[test]
    fn test_block_header() {
        let bytes = [0x0u8, 0x0];
        let hi = Sha3_256::digest(bytes);
        let hash = H256::from_slice(&hi);

        let header = BlockHeader {
            height: BlockHeight(0x12345678),
            time: Some(Timestamp(0x12345679)),
            parent_hash: BlockHash(hash.clone()),
            txs_hash: MerkleHash(hash.clone()),
            app_hash: MerkleHash(hash.clone()),
            proposer: NodeAddress(H160::from(hash)),
        };

        let by = header.to_bytes().unwrap();

        let header1 = BlockHeader::from_bytes(&by).unwrap();

        assert_eq!(header.height, header1.height);
        assert_eq!(header.time, header1.time);
        assert_eq!(header.parent_hash, header1.parent_hash);
        assert_eq!(header.txs_hash, header1.txs_hash);
        assert_eq!(header.app_hash, header1.app_hash);
        assert_eq!(header.proposer, header1.proposer);
    }
}
