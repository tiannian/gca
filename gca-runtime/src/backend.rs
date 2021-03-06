use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::{FuncDefine, ModuleInfo, Result, Val};

pub trait Module: Sized {
    fn load_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>;
}

pub trait Instance: Sized + 'static {
    type Module: Module;

    type Memory: Memory;

    fn call_func(&mut self, name: &str, parmas: &[Val]) -> Result<Option<Val>>;

    fn call_func_for_host(&mut self, name: &str, parmas: &[Val]) -> Result<Option<Val>>;

    fn get_memory(&self, name: &str) -> Option<Self::Memory>;

    fn get_host(&self, name: &str) -> Option<&dyn Host<Self>>;

    fn get_host_mut(&mut self, name: &str) -> Option<&mut dyn Host<Self>>;
}

pub trait Host<I: Instance>: Downcast + 'static {
    fn resolve_functions(&self) -> &[FuncDefine];

    fn set_instance(&mut self, instance: I);

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn Debug + Sync + Send>>;
}

downcast_rs::impl_downcast!(Host<I> where I: Instance);

pub trait Memory: Clone {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;

    fn write(&self, offset: usize, buffer: &[u8]) -> Result<()>;
}

pub trait Backend {
    type Module: Module;

    type Instance: Instance<Module = Self::Module, Memory = Self::Memory>;

    type Memory: Memory + 'static;

    // Create new wasm backend from host functions
    fn new() -> Self;

    fn add_host(&mut self, name: &str, host: impl Host<Self::Instance>);

    fn instance(
        self,
        module: &Self::Module,
        deps: &[ModuleInfo<'_, Self::Module>],
    ) -> Result<Self::Instance>;
}
