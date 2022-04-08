use crate::{FuncDefine, Host, Memory, Val};

pub enum LoggerError {}

pub struct Logger<M> {
    func_def: Vec<FuncDefine>,
    memory: Option<M>,
}

impl<M> Logger<M> {
    pub fn new() -> Self {
        let f = FuncDefine { name: "_gca_log", parmas: vec![], ret: None };

        let func_def = vec![f];

        Self {
            func_def,
            memory: None,
        }
    }
}

impl<M: Memory + 'static> Host<M> for Logger<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_memory(&mut self, memory: M) {
        self.memory = Some(memory);
    }

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn std::fmt::Debug + Sync + Send>> {
        Ok(None)
    }
}
