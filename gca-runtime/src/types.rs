#[derive(Debug, Clone)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
}

#[derive(Debug, Clone)]
pub enum ValTy {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub struct FuncDefine {
    pub name: &'static str,
    pub parmas: Vec<ValTy>,
    pub ret: Option<ValTy>,
}

pub struct HostInfo<'a, H> {
    pub name: &'a str,
    pub host: H,
}

pub struct ModuleInfo<'a, M> {
    pub name: &'a str,
    pub module: &'a M,
}
