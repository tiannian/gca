use alloc::vec::Vec;
use primitive_types::{H160, H256};

macro_rules! impl_traits {
    ($ty:ty, $len:expr ) => {
        impl AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                self.0.as_bytes()
            }
        }

        impl AsMut<[u8]> for $ty {
            fn as_mut(&mut self) -> &mut [u8] {
                self.0.as_bytes_mut()
            }
        }

        impl From<[u8; $len]> for $ty {
            fn from(b: [u8; $len]) -> Self {
                Self(From::from(b))
            }
        }
    };
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHash(pub H256);

impl_traits!(BlockHash, 32);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Txhash(pub H256);

impl_traits!(Txhash, 32);

#[derive(Debug, Default, Clone)]
pub struct MemoOperation(pub u32);

#[derive(Debug, Default, Clone)]
pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputOperation(pub u32);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Amount(pub u64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MerkleHash(pub H256);

impl_traits!(MerkleHash, 32);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Timestamp(pub i64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NodeAddress(pub H160);

impl_traits!(NodeAddress, 20);
