use crate::Result;

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

pub trait ToBytes {
    type Bytes: AsRef<[u8]>;

    fn to_bytes(&self) -> Result<Self::Bytes>;
}
