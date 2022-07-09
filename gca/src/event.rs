use alloc::{vec::Vec, string::String};

#[derive(Clone, Debug)]
pub struct EventAttribute {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub index: bool,
}

#[derive(Debug, Default)]
pub struct Event {
    pub name: String,
    pub attrs: Vec<EventAttribute>,
}


