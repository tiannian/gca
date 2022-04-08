use crate::{FuncDefine, Host, Memory, Val, ValTy};

pub struct Env<M> {
    chain_id: &'static str,
    func_def: Vec<FuncDefine>,
    memory: Option<M>,
}

impl<M: Memory + 'static> Env<M> {
    pub fn new(chain_id: &'static str) -> Self {
        let f = FuncDefine {
            name: "_gca_env_get_chain_id",
            parmas: vec![],
            ret: Some(ValTy::I32),
        };

        let func_def = vec![f];

        Self {
            func_def,
            chain_id,
            memory: None,
        }
    }
}

impl<M: Memory + 'static> Host<M> for Env<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_memory(&mut self, memory: M) {}

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> Result<Option<Val>, Box<dyn std::fmt::Debug + Sync + Send>> {
        Ok(None)
    }
}
