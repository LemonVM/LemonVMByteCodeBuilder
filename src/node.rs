use super::opcode_builder::*;
use liblemonvm::binary::{constant::Constant, debug::{DebugVariable, DebugVariableTable}};
use std::collections::BTreeMap;

trait BytecodeGen {
    fn gen(&self) -> Vec<u8>;
}

struct Module {
    // file: std::fs::File,
}

// struct ExceptionTable (pub Vec<ExceptionScope>);

#[repr(u8)]
pub enum FunctionType {
    Function = 0x00,
    // currently disabled
    Generator,
    // currently disabled
    AsyncFunction,
    // currently disabled
    AsyncGenerator,
}
#[derive(Debug)]
pub struct Function {
    // function_type: FunctionType,
    // indexed by uuid
    // just used to build arguments object
    // args_count: u8,
    // max_registers: u16,

    pub constant_pool_ref: *mut BTreeMap<u16,Constant>,

    pub debug_variable_table: DebugVariableTable,
    pub register_alloc_table: Vec<(u16, Vec<Variable>)>,

    // exception_table: Vec<Scope>,
    // scope: Scope,
}

// impl Function {
//     fn new(
//         function_type: FunctionType
//     ) -> Self {
//         Function {
//             function_type,
//             args_count:0,
//             max_registers: 0,
//             exception_table: Vec::new(),
//             scope: Scope::default(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Variable{
    pub name: String,
    pub start_pc: u16,
    pub end_pc:u16,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub father: Option<*mut Scope>,
    pub start_pc: u16,
    pub end_pc: u16,
    pub variable_table: Vec<Variable>,
    pub is_exception_scope: bool,
    pub code: Vec<(u16,OpCodeFormat)>,
}

impl Scope {
    pub fn start(
        father: Option<*mut Self>,
        is_exception_scope: bool,
        with_local_variable_scope: bool,
    ) -> Self {
        let mut start_pc = 0;
        let mut end_pc = 0;
        match father {
            Some(father) => unsafe {
                start_pc = (&*father).end_pc;
                end_pc = (&*father).end_pc;
            },
            None => {
                start_pc = 0;
                end_pc = 0;
            }
        }
        Scope {
            father,
            start_pc,
            end_pc,
            variable_table: vec![],
            is_exception_scope,
            code: vec![],
        }
    }

    pub fn alloc_register(&self, fun: &mut Function) {
        let mut len = fun.register_alloc_table.len();
        for var in self.variable_table.iter() {
            let Variable{name, start_pc, end_pc} = var;
            let mut i = 0;
            loop {
                if len == i{
                    fun.register_alloc_table.push((i as u16,vec![var.clone()]));

                    let pool = unsafe{&mut *fun.constant_pool_ref};
                    let plen = pool.len();
                    pool.insert(plen as u16, Constant::String(name.clone()));
                    fun.debug_variable_table.table.push(DebugVariable{
                        name: plen as u16,
                        start_pc: *start_pc,
                        end_pc: *end_pc,
                        register: i as u16,
                    });

                    len += 1;
                    i = 0;
                    break;
                }
                let (r, vars) =
                    &mut fun.register_alloc_table[i];

                if *start_pc > vars.last().unwrap().end_pc {
                    vars.push(var.clone());

                    let pool = unsafe{&mut *fun.constant_pool_ref};
                    let plen = pool.len();
                    pool.insert(plen as u16, Constant::String(name.clone()));
                    fun.debug_variable_table.table.push(DebugVariable{
                        name: plen as u16,
                        start_pc: *start_pc,
                        end_pc: *end_pc,
                        register: i as u16,
                    });
                    i = 0;
                    break;
                }
                i+=1;
            }
        }
    }

    pub fn add_variable(&mut self, name: Option<String>, scope: *mut Scope) {
        let offset = self.variable_table.len();
        let name = match name {
            Some(name) => name,
            None => offset.to_string(),
        };
        //FIXME: 这个不对👇 应该修好了
        unsafe {
            (&mut *scope)
                .variable_table
                .push(Variable{
                    name: name, 
                    start_pc:self.end_pc,
                    end_pc:self.end_pc
                })
        };
    }

    pub fn add_code(&mut self, code: &mut Vec<(u16,OpCodeFormat)>) {
        self.end_pc += code.len() as u16;
        self.code.append(code);
    }


    pub fn adjust_end_pc(&mut self) {
        match self.father {
            Some(father) => unsafe {
                (&mut *father).end_pc = self.end_pc;
                (&mut *father).adjust_end_pc();
            },
            None => {}
        }
    }

    pub fn end(&mut self,fun: &mut Function) {
        self.adjust_end_pc();
        // correct end_pc for variable table
        let end_pc = self.end_pc;
        for i in 0..self.variable_table.len() {
            self.variable_table[i].end_pc = end_pc;
            for j in i..self.variable_table.len() {
                if self.variable_table[i].name == self.variable_table[j].name && i != j {
                    self.variable_table[i].end_pc = self.variable_table[j].start_pc;
                    break;
                }
            }
        }
        self.alloc_register(fun);
        match self.father {
            Some(father) => unsafe {
                (&mut *father).code.append(&mut self.code);
            },
            None => {}
        }
    }
}
