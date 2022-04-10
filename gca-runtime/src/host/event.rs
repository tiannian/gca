use std::{collections::BTreeMap, fmt::Debug, string::FromUtf8Error, sync::Arc};

use crate::{FuncDefine, Host, Instance, Memory, Val, ValTy};

#[derive(Debug)]
pub enum EmiterError {
    NoMethod,
    NoArguments,
    ErrWasmType,
    MemoryReadError,
    FromUtf8Error,
    NoInstance,
    NoMemory,
}

impl From<FromUtf8Error> for EmiterError {
    fn from(_: FromUtf8Error) -> Self {
        Self::FromUtf8Error
    }
}

impl From<EmiterError> for Box<dyn Debug + Sync + Send> {
    fn from(e: EmiterError) -> Self {
        Box::new(e)
    }
}

#[derive(Clone)]
pub struct EventAttribute {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub index: bool,
}

pub struct EventEmiter<M> {
    func_def: Arc<Vec<FuncDefine>>,
    instance: Option<M>,
    events: BTreeMap<String, Vec<EventAttribute>>,
}

impl<M> Clone for EventEmiter<M> {
    fn clone(&self) -> Self {
        Self {
            func_def: self.func_def.clone(),
            instance: None,
            events: self.events.clone(),
        }
    }
}

impl<M> EventEmiter<M> {
    pub fn new() -> Self {
        let f = FuncDefine {
            name: "_gca_emit",
            parmas: vec![
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
            ],
            ret: None,
        };

        let func_def = Arc::new(vec![f]);

        Self {
            func_def,
            instance: None,
            events: BTreeMap::new(),
        }
    }
}

impl<M: Instance + 'static> Host<M> for EventEmiter<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_instance(&mut self, instance: M) {
        self.instance = Some(instance);
    }

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn std::fmt::Debug + Sync + Send>> {
        if name != "_gca_log" {
            return Err(Box::new(EmiterError::NoMethod));
        }

        let name_ptr = args.get(0).ok_or(EmiterError::NoArguments)?;
        let name_len = args.get(1).ok_or(EmiterError::NoArguments)?;
        let key_ptr = args.get(2).ok_or(EmiterError::NoArguments)?;
        let key_len = args.get(3).ok_or(EmiterError::NoArguments)?;
        let val_ptr = args.get(4).ok_or(EmiterError::NoArguments)?;
        let val_len = args.get(5).ok_or(EmiterError::NoArguments)?;
        let index = args.get(6).ok_or(EmiterError::NoArguments)?;

        let memory = self
            .instance
            .as_ref()
            .ok_or(EmiterError::NoInstance)?
            .get_memory("memory")
            .ok_or(EmiterError::NoMemory)?;

        let name = get_string(name_len, name_ptr, &memory)?;
        let key = get_bytes(key_ptr, key_len, &memory)?;
        let value = get_bytes(val_ptr, val_len, &memory)?;
        let index = val_to_i32(index)? == 0;

        let attr = EventAttribute {
            key, value, index
        };

        if let Some(v) = self.events.get_mut(&name) {
            v.push(attr);
        } else {
            let v = vec![attr];
            self.events.insert(name, v);
        }

        Ok(None)
    }
}

fn val_to_i32(v: &Val) -> Result<i32, EmiterError> {
    match v {
        Val::I32(i) => Ok(*i),
        _ => Err(EmiterError::ErrWasmType),
    }
}

fn get_bytes(ptr: &Val, len: &Val, memory: &impl Memory) -> Result<Vec<u8>, EmiterError> {
    let ptr = val_to_i32(ptr)?;
    let len = val_to_i32(len)?;

    let mut buf = Vec::<u8>::with_capacity(len as usize);

    buf.resize(len as usize, 0);

    memory
        .read(ptr as usize, &mut buf)
        .map_err(|_| EmiterError::MemoryReadError)?;

    Ok(buf)
}

fn get_string(ptr: &Val, len: &Val, memory: &impl Memory) -> Result<String, EmiterError> {
    let buf = get_bytes(ptr, len, memory)?;

    Ok(String::from_utf8(buf)?)
}
