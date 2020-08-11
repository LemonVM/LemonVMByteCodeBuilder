use super::irbuilder::*;

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

pub struct Function {
    function_type: FunctionType,
    // indexed by uuid
    // just used to build arguments object
    args_count: u8,
    max_registers: u16,

    register_alloc_table: Vec<(u16, u16, u16)>,

    exception_table: Vec<Scope>,
    scope: Scope,
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
pub struct Scope {
    pub father: Option<*mut Scope>,
    pub start_pc: u16,
    pub end_pc: u16,
    pub variable_table: Vec<(String, u16, u16)>,
    pub is_exception_scope: bool,
    pub shit: Vec<u8>,
    pub code: Vec<HighCode>,
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
            shit: vec![],
            code: vec![],
        }
    }

    pub fn alloc_register(&self, fun: &mut Function) {
        let mut len = fun.register_alloc_table.len();
        for (_, start_pc, end_pc) in self.variable_table.iter() {
            let mut i = 0;
            loop {
                let (spc, epc, r) =
                    &mut fun.register_alloc_table[i];
                if i == len {
                    fun.register_alloc_table.push((*start_pc, *end_pc, i as u16));
                    len+=1;
                    break;
                } else if start_pc > epc {
                    *spc = *start_pc;
                    *epc = *end_pc;
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
                .push((name, self.end_pc, self.end_pc))
        };
    }

    pub fn add_code(&mut self, code: &mut Vec<HighCode>) {
        self.end_pc += (code.len() / 8) as u16;
        self.code.append(code);
    }

    pub fn add_shit(&mut self, code: &mut Vec<u8>) {
        self.end_pc += (code.len() / 8) as u16;
        self.shit.append(code);
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

    pub fn end(&mut self) {
        self.adjust_end_pc();
        // correct end_pc for variable table
        let end_pc = self.end_pc;
        for i in 0..self.variable_table.len() {
            self.variable_table[i].2 = end_pc;
            for j in i..self.variable_table.len() {
                if self.variable_table[i].0 == self.variable_table[j].0 && i != j {
                    self.variable_table[i].2 = self.variable_table[j].1;
                    break;
                }
            }
        }
        match self.father {
            Some(father) => unsafe {
                (&mut *father).shit.append(&mut self.shit);
            },
            None => {}
        }
    }
}
