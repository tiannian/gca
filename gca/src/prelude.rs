use alloc::vec::Vec;
use bytes::BufMut;

use crate::Result;

pub trait BytesSize {
    fn bytes_size() -> usize;
}

pub trait FromBytes: Sized + BytesSize {
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

pub trait ToBytes: BytesSize {
    fn to_bytes(&self, buf: &mut impl BufMut) -> Result<()>;
}

pub trait IntoBytes: ToBytes {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(Self::bytes_size());

        self.to_bytes(&mut buf)?;

        Ok(buf)
    }
}
