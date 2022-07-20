use alloc::{vec::Vec, string::String};

use crate::{Hashable, MerkleHash};

#[derive(Clone, Debug)]
pub struct EventAttribute {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub index: bool,
}

impl Hashable for EventAttribute {
    fn get_hash<D: digest::Digest<OutputSize = digest::consts::U32>>(&self) -> MerkleHash {
        let mut hasher = D::new();

        hasher.update(&self.key);
        hasher.update(&self.value);

        MerkleHash::from_slice(&hasher.finalize())
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    pub name: String,
    pub attrs: Vec<EventAttribute>,
}

impl Hashable for Event {
    fn get_hash<D: digest::Digest<OutputSize = digest::consts::U32>>(&self) -> MerkleHash {
        let mut hasher = D::new();

        hasher.update(&self.name);

        for attr in &self.attrs {
            hasher.update(&attr.get_hash::<D>());
        }

        MerkleHash::from_slice(&hasher.finalize())
    }
}

