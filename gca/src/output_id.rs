use alloc::{string::String, format};
use bytes::{Buf, BufMut};

use crate::{Txhash, Result, Error, FromBytes, utils, ToBytes, BytesSize, IntoBytes};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputId {
    pub txhash: Txhash,
    pub n: u64,
}

impl OutputId {
    pub fn to_hex(&self) -> String {
        let inner = hex::encode(self.txhash.0);

        let s = format!("0x{}:{}", inner, self.n);

        s
    }

    pub fn from_hex(s: &str) -> Result<Self> {
        let prefix = &s[..2];

        if prefix != "0x" {
            return Err(Error::ErrPrefix("0x"));
        }

        let inner = &s[2..66];

        let txhash_bytes = hex::decode(inner)?;

        let txhash = Txhash::from_slice(&txhash_bytes);

        let n_str = &s[67..];

        let n = n_str.parse()?;

        Ok(OutputId { txhash, n })
    }
}

impl BytesSize for OutputId {
    fn bytes_size() -> usize {
        32 + 8
    }
}

impl FromBytes for OutputId {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = utils::Bytes::new(bytes);

        let mut txhash = Txhash::default();

        reader.copy_to_slice(txhash.as_mut());

        let n = reader.get_u64();

        Ok(OutputId { txhash, n })
    }
}

impl ToBytes for OutputId {
    fn to_bytes(&self, buff: &mut impl BufMut) -> Result<()> {

        buff.put_slice(self.txhash.as_ref());
        buff.put_u64(self.n);

        Ok(())
    }
}

impl IntoBytes for OutputId {}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_oid() -> OutputId {
        let tx_ = [8u8; 32];

        OutputId {
            txhash: Txhash::from(tx_),
            n: 13,
        }
    }

    #[test]
    fn test_output_id_parse() {
        let output_id = build_oid();

        let s = output_id.to_hex();

        let _oid = OutputId::from_hex(&s).unwrap();
    }

    #[test]
    fn test_output_id() {
        let oid = build_oid();

        let by = oid.into_bytes().unwrap();

        let oid1 = OutputId::from_bytes(&by).unwrap();

        assert_eq!(oid, oid1);
    }
}
