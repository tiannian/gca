use bytes::Buf;

pub struct Bytes<'a> {
    pos: usize,
    inner: &'a [u8],
}

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            pos: 0,
            inner: bytes,
        }
    }
}

impl<'a> Buf for Bytes<'a> {
    fn remaining(&self) -> usize {
        self.inner.len() - self.pos
    }

    fn chunk(&self) -> &[u8] {
        &self.inner[self.pos..]
    }

    fn advance(&mut self, cnt: usize) {
        self.pos += cnt;
    }
}
