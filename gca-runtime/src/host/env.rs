use std::{fmt::Debug, sync::Arc};

use crate::{FuncDefine, Host, Instance, Memory, Val, ValTy};

#[derive(Debug)]
pub enum EnvError {
    ErrMethodName,
    ErrNoInstance,
    ErrNoMemory,
    ErrAllocFormatError,
    CallFuncError(crate::Error),
    MemoryError(crate::Error),
}

impl From<EnvError> for Box<dyn Debug + Sync + Send> {
    fn from(e: EnvError) -> Self {
        Box::new(e)
    }
}

pub struct Env<M> {
    chain_id: &'static str,
    func_def: Arc<Vec<FuncDefine>>,
    instance: Option<M>,
}

impl<M> Clone for Env<M> {
    fn clone(&self) -> Self {
        Self {
            chain_id: self.chain_id,
            func_def: self.func_def.clone(),
            instance: None,
        }
    }
}

impl<M> Env<M> {
    pub fn new(chain_id: &'static str) -> Self {
        let f = FuncDefine {
            name: "_gca_env_get_chain_id",
            parmas: vec![],
            ret: Some(ValTy::I32),
        };

        let func_def = Arc::new(vec![f]);

        Self {
            func_def,
            chain_id,
            instance: None,
        }
    }
}

impl<M: Instance + 'static> Host<M> for Env<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_instance(&mut self, instance: M) {
        self.instance = Some(instance);
    }

    //     fn as_any(&self) -> &dyn Any {
    // self
    //     }

    fn call_func(
        &mut self,
        name: &str,
        _args: &[Val],
    ) -> Result<Option<Val>, Box<dyn std::fmt::Debug + Sync + Send>> {
        if name != "_gca_env_get_chain_id" {
            return Err(EnvError::ErrMethodName.into());
        }

        let instance = self.instance.as_mut().ok_or(EnvError::ErrNoInstance)?;

        let args = [Val::I32(self.chain_id.len() as i32)];

        if let Some(Val::I32(ptr)) = instance
            .call_func_for_host("_gca_env_alloc", &args)
            .map_err(EnvError::CallFuncError)?
        {
            let memory = instance.get_memory("memory").ok_or(EnvError::ErrNoMemory)?;
            memory
                .write(ptr as usize, self.chain_id.as_bytes())
                .map_err(EnvError::MemoryError)?;
            Ok(Some(Val::I32(ptr)))
        } else {
            Err(EnvError::ErrAllocFormatError.into())
        }
    }
}
