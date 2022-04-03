use std::fmt::Debug;

use crate::{FuncDefine, ModuleInfo, Result, Val};

pub trait Module: Sized {
    fn load_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>;
}

pub trait Instance: Sized {
    type Module: Module;

    type Memory: Memory;

    fn call_func(&mut self, name: &str, parmas: &[Val]) -> Result<Option<Val>>;

    fn get_memory(&self, name: &str) -> Option<Self::Memory>;
}

pub trait Host<M: Memory> {
    fn resolve_functions(&self) -> &[FuncDefine];

    fn set_memory(&mut self, memory: M);

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn Debug>>;
}

pub trait Memory {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;

    fn write(&self, offset: usize, buffer: &[u8]) -> Result<()>;
}

pub trait Backend {
    type Module: Module;

    type Instance: Instance<Module = Self::Module, Memory = Self::Memory>;

    type Memory: Memory;

    // Create new wasm backend from host functions
    fn new() -> Self;

    fn add_host(&mut self, name: &str, host: impl Host<Self::Memory>);

    fn instance(
        &mut self,
        module: &Self::Module,
        deps: &[ModuleInfo<'_, Self::Module>],
    ) -> Result<Self::Instance>;
}
