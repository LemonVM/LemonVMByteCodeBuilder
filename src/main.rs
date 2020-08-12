#![feature(linked_list_cursors)]
mod context;
mod opcode_builder;
mod node;

use node::*;
use liblemonvm::binary::debug::DebugVariableTable;
use std::collections::BTreeMap;
use opcode_builder::{print_generated_code, OpCodeBuilder};

fn main() {
    let mut cp = BTreeMap::new();
    let mut cb = OpCodeBuilder::new();
    let mut function = Function{ constant_pool_ref: &mut cp, debug_variable_table: DebugVariableTable{table:vec![]}, register_alloc_table: vec![]};
    let mut s1 = Scope::start(None, false, true);
    let s1_ptr = &mut s1 as *mut _;
    s1.add_variable(Some("a".to_string()), s1_ptr);
    cb.immu16("a".to_string(), 65534);
    s1.add_code(&mut cb.get_and_clean());
    let mut s2 = Scope::start(Some(&mut s1), false, false);
    s2.add_variable(Some("a".to_string()), &mut s1);
    cb.immu32("a".to_string(), 4_294_967_294);
    s2.add_code(&mut cb.get_and_clean());
    s2.add_variable(Some("b".to_string()), &mut s1);
    cb.immu64("b".to_string(), 18_446_744_073_709_551_614);
    s2.add_code(&mut cb.get_and_clean());
    s2.add_variable(Some("c".to_string()), &mut s1);
    cb.add("a".to_string(), "b".to_string(), "c".to_string());
    s2.add_code(&mut cb.get_and_clean());
    s2.add_variable(Some("b".to_string()), &mut s1);
    cb.inc("b".to_string());
    s2.add_code(&mut cb.get_and_clean());
    s2.add_variable(Some("b".to_string()), &mut s1);
    cb.nop();
    s2.add_code(&mut cb.get_and_clean());
    s2.end(&mut function);
    s1.end(&mut function);
    
    cb.data = s1.code.clone();
    println!("\n\n{:?}\n\n", s1.variable_table);
    println!("{:?}", function.register_alloc_table);

    print_generated_code(&cb.gen_code(&function));
}
