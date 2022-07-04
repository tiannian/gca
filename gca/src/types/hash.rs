use primitive_types::{H160, H256};

macro_rules! impl_traits {
    ($ty:ty, $len:expr, $inner:ty) => {
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

        impl $ty {
            pub fn from_slice(b: &[u8]) -> Self {
                Self(<$inner>::from_slice(b))
            }
        }
    };
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHash(pub H256);

impl_traits!(BlockHash, 32, H256);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Txhash(pub H256);

impl_traits!(Txhash, 32, H256);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MerkleHash(pub H256);

impl_traits!(MerkleHash, 32, H256);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NodeAddress(pub H160);

impl_traits!(NodeAddress, 20, H160);
