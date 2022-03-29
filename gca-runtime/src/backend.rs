use std::path::Path;

use crate::{Result, Val, ValTy};

pub trait Module: Sized {
    fn load_file(file: impl AsRef<Path>) -> Result<Self>;

    fn load_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>;
}

pub trait Instance: Sized {
    type Module: Module;

    type Function: Function;

    type Memory: Memory;

    type Host: Host;

    fn new(module: &Self::Module, deps: &[Self::Module], host: &[Self::Host]) -> Result<Self>;

    fn get_function(&self) -> Option<Self::Function>;

    fn get_memory(&self) -> Option<Self::Memory>;
}

pub trait Host {
    fn resolve_func(args: &[ValTy], ret: Option<ValTy>) -> Result<usize>;

    fn call_func(args: &[Val]) -> Result<Val>;
}

pub trait Function {
    fn call(args: &[Val]) -> Option<Val>;
}

pub trait Memory {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;

    fn write(&self, offset: usize, buffer: &[u8]) -> Result<()>;
}

pub trait Backend {
    type Module: Module;

    type Instance: Instance<
        Module = Self::Module,
        Function = Self::Function,
        Memory = Self::Memory,
        Host = Self::Host,
    >;

    type Function: Function;

    type Memory: Memory;

    type Host: Host;
}
