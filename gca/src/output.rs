use alloc::vec::Vec;
use bytes::{Buf, BufMut};

use crate::{
    utils, Amount, BytesSize, Error, FromBytes, OutputId, OutputOperation, Result, ToBytes, MerkleHash, IntoBytes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KV {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputData {
    NativeToken(Amount),
    Data(Vec<u8>),
    Map(Vec<KV>, MerkleHash),
}

impl Default for OutputData {
    fn default() -> Self {
        Self::NativeToken(Amount::default())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Verifier {
    pub output_id: OutputId,
    pub gas_limit: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OutputCore {
    pub data: OutputData,
    pub locker: OutputId,
    pub verifier: Option<Verifier>,
    pub owner: Vec<u8>,
}

impl BytesSize for OutputCore {
    fn bytes_size() -> usize {
        4 + OutputId::bytes_size() * 2 + 1
    }
}

impl FromBytes for OutputCore {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let s = Self::bytes_size();

        if bytes.len() < s {
            return Err(Error::BytesSizeError(s, bytes.len()));
        }

        let mut reader = utils::Bytes::new(bytes);

        let locker = OutputId::from_bytes(&reader.copy_to_bytes(OutputId::bytes_size()))?;

        let verifier = {
            let flag = reader.get_u8();
            if flag == 0 {
                None
            } else {
                let gas_limit = reader.get_u64();
                let output_id =
                    OutputId::from_bytes(&reader.copy_to_bytes(OutputId::bytes_size()))?;

                Some(Verifier {
                    gas_limit,
                    output_id,
                })
            }
        };

        let owner_len = reader.get_u32() as usize;

        let owner = reader.copy_to_bytes(owner_len).to_vec();

        let data = {
            let flag = reader.get_u8();

            match flag {
                0 => OutputData::NativeToken(Amount(reader.get_u64())),
                1 => OutputData::Data(reader.copy_to_bytes(reader.remaining()).to_vec()),
                2 => {
                    let mut merkle = MerkleHash::default();
                    reader.copy_to_slice(merkle.as_mut());
                    OutputData::Map(Vec::new(), merkle)
                }
                _ => return Err(Error::InvaildOutputType(flag)),
            }
        };

        Ok(Self {
            locker,
            verifier,
            data,
            owner,
        })
    }
}

impl ToBytes for OutputCore {
    fn to_bytes(&self, buff: &mut impl BufMut) -> Result<()> {
        self.locker.to_bytes(buff)?;

        if let Some(d) = &self.verifier {
            buff.put_u8(1);
            buff.put_u64(d.gas_limit);
            d.output_id.to_bytes(buff)?;
        } else {
            buff.put_u8(0);
        }

        buff.put_u32(self.owner.len() as u32);
        buff.put_slice(&self.owner);

        match &self.data {
            OutputData::NativeToken(v) => {
                buff.put_u8(0);
                buff.put_u64(v.0)
            }
            OutputData::Data(v) => {
                buff.put_u8(1);
                buff.put_slice(v)
            }
            OutputData::Map(_, merkle) => {
                buff.put_u8(2);
                buff.put_slice(merkle.as_ref());
            }
        }

        Ok(())
    }
}

impl IntoBytes for OutputCore {}

#[derive(Debug, Default, Clone)]
pub struct Output {
    pub core: OutputCore,
    pub operation: OutputOperation,
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_output() {
        let oid = OutputId::from_hex("0x1020304:1").unwrap();

        let verifier = Verifier {
            output_id: oid.clone(),
            gas_limit: 19,
        };

        let core = OutputCore {
            data: OutputData::Data(vec![0, 1, 2]),
            locker: oid,
            verifier: Some(verifier),
            owner: vec![1,2,3,4,5],
        };

        let by = core.into_bytes().unwrap();

        let core1 = OutputCore::from_bytes(&by).unwrap();

        assert_eq!(core, core1);

    }
}
