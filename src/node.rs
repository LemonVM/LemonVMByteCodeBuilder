
use std::collections::{LinkedList, HashMap};

trait Node {

}

struct Module {
    file: std::fs::File,
}

struct ExceptionTable (pub Vec<ExceptionScope>);

#[repr(u8)]
pub enum FunctionType {
    Function = 0x00,
    // currently disabled
    Generator,
    // currently disabled
    AsyncFunction,
    // currently disabled
    AsyncGenerator
}

pub struct Function {
    function_type: FunctionType,
    // indexed by uuid
    // just used to build arguments object
    args_count: u8,
    max_registers: u16,

    exception_table: Option<ExceptionTable>,
    scope: Scope,
}

impl Function {
    fn new(
        function_type: FunctionType
    ) -> Self {
        Function {
            function_type,
            args_count:0,
            max_registers: 0,
            exception_table: None,
            scope: Scope::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct ScopeInner {
    variable_table: HashMap<String, u16>

}

#[derive(Debug, Clone)]
struct Scope (pub LinkedList<ScopeInner>);

#[derive(Debug, Clone)]
struct ExceptionScope (pub Scope);


impl ScopeInner {
    fn new(vt: &LinkedList<String>) -> Self {
        ScopeInner {
            variable_table: HashMap::new()
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        let mut r = LinkedList::new();
        r.push_back(ScopeInner::default());
        Scope(r)
    }
}

impl Scope {
    fn new(&self) -> Self {
        let mut r = self.0.clone();
        r.push_back(ScopeInner::default());
        Scope(r)
    }

    fn add_variable(& mut self, name: Option<String>) {
        let current_scope = self.0.iter_mut().last().unwrap();
        let offset = current_scope.variable_table.len();
        let name = match name {
            Some(name) => name,
            None => offset.to_string()
        };
        current_scope.variable_table.insert(name, offset as u16);
    }

    fn add_code() {
        
    }
}
