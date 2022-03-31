use std::fmt::{Debug, Display};

use crate::{FuncDefine, HostInfo, ModuleInfo, Result, Val};

pub trait Module: Sized {
    fn load_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>;
}

pub trait Instance<H: Host>: Sized {
    type Module: Module;

    type Memory: Memory;

    fn call_func(&self, name: &str, parmas: &[Val]) -> Result<Option<Val>>;

    fn get_memory(&self, name: &str) -> Option<Self::Memory>;
}

pub trait Host {
    type Error: Debug + Display;

    fn resolve_functions(&self) -> &[FuncDefine];

    fn call_func(&mut self, name: &str, args: &[Val]) -> std::result::Result<Option<Val>, Self::Error>;
}

pub trait Memory {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;

    fn write(&self, offset: usize, buffer: &[u8]) -> Result<()>;
}

pub trait Backend<H: Host> {
    type Module: Module;

    type Instance: Instance<H, Module = Self::Module, Memory = Self::Memory>;

    type Memory: Memory;

    type Host: Host;

    // Create new wasm backend from host functions
    fn new(host: &[HostInfo<'_, H>]) -> Self;

    fn instance(
        &mut self,
        module: &Self::Module,
        deps: &[ModuleInfo<'_, Self::Module>],
    ) -> Result<Self::Instance>;
}
