#![allow(unused)]

pub struct Linker {
    functions: Vec<(u32, String)>,
}

impl Linker {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }

    pub fn bind<F>(&mut self, id: u32, name: &str, _f: F) -> &mut Self {
        self.functions.push((id, name.to_string()));
        self
    }
}

impl Default for Linker {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

pub enum ExecutionResult {
    Success,
    Revert,
    OutOfGas,
    Trap,
}

pub enum HostFnResult {
    Success,
    Revert(String),
}
