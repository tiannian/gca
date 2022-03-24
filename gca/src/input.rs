use alloc::{vec::Vec, format, string::String};
use primitive_types::H256;

use crate::{Txhash, Result, Error};

#[derive(Debug, Default)]
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
            return Err(Error::ErrPrefix);
        }

        let inner = &s[2..66];

        let txhash_bytes = hex::decode(inner)?;

        let txhash = Txhash(H256::from_slice(&txhash_bytes));

        let n_str = &s[67..];

        let n = u64::from_str_radix(n_str, 10)?;

        Ok(OutputId {
            txhash,
            n
        })
    }
}

pub enum InputOperation {
    Input(u32),
    Reference,
}

pub struct Input {
    pub output_id: OutputId,
    pub unlock: Vec<u8>,
    pub operation: InputOperation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_id() {
        let tx_ = [8u8; 32];

        let output_id = OutputId {
            txhash: Txhash(H256::from(tx_)),
            n: 13,
        };

        let s = output_id.to_hex();

        let _oid = OutputId::from_hex(&s).unwrap();
    }
}

