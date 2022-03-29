use crate::{Memory, Result};

pub struct WasmiMemory {
    pub m: wasmi::MemoryRef,
}

impl Memory for WasmiMemory {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()> {
        let offset: u32 = offset.try_into()?;

        self.m.get_into(offset, buffer)?;
        Ok(())
    }

    fn write(&self, offset: usize, buffer: &[u8]) -> Result<()> {
        let offset: u32 = offset.try_into()?;

        self.m.set(offset, buffer)?;

        Ok(())
    }
}
