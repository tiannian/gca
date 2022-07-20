use alloc::{string::String, vec::Vec};

use crate::{Event, MerkleHash, Txhash};

#[derive(Debug, Default)]
pub struct Receipt {
    /// Hash of transaction.
    pub txhash: Txhash,

    /// Total code of all segment.
    pub total_code: i32,

    /// Total gas of all segment.
    pub total_gas: i64,

    /// Receipts of unlock output.
    pub unlock: Vec<ReceiptSegment>,

    /// Receipts of operate transaction.
    pub operate: Vec<ReceiptSegment>,

    /// Receipt of verify output.
    pub verify: Vec<ReceiptSegment>,
}

impl Receipt {
    /// Create receipt
    pub fn new(txhash: Txhash, uns: Vec<ReceiptSegment>, ops: Vec<ReceiptSegment>, vs: Vec<ReceiptSegment>) -> Self {
        let mut total_code = 0;
        let mut total_gas = 0;

        for receipt in &uns {
            total_code ^= receipt.code;
            total_gas += receipt.gas;
        }

        for receipt in &ops {
            total_code ^= receipt.code;
            total_gas += receipt.gas;
        }

        for receipt in &vs {
            total_code ^= receipt.code;
            total_gas += receipt.gas;
        }

        Self {
            txhash,
            total_code,
            total_gas,
            unlock: uns,
            operate: ops,
            verify: vs,
        }
    }
}

#[derive(Debug, Default)]
pub struct ReceiptSegment {
    /// Receipt code for this segment, 0 is normal.
    pub code: i32,

    /// Log string.
    /// Note: This field will not compute on merkle.
    pub log: String,

    /// Gas of this segment.
    pub gas: i64,

    /// Events of this segment.
    /// This file will store standalone.
    pub events: Vec<Event>,

    /// Hash of events.
    pub events_hash: MerkleHash,

    /// Return data
    pub data: Vec<u8>,
}

