#![feature(linked_list_cursors)]
mod context;
mod node;

use node::*;

fn main() {
    let mut s1 = Scope::start(None,false, true);
    let s1_ptr = &mut s1 as *mut _;
    s1.add_variable(Some("a".to_string()), s1_ptr);
    s1.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    let mut s2 = Scope::start(Some(&mut s1),false,false);
    s2.add_variable(Some("b".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.add_variable(Some("b".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.add_variable(Some("c".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.add_variable(Some("b".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.add_variable(Some("b".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.add_variable(Some("c".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.add_variable(Some("b".to_string()), &mut s1);
    s2.add_code(&mut vec![0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    s2.end();
    // let mut s3 = Scope::start(Some(&mut s1),false);
    // s3.add_variable(Some("jiba".to_string()));
    // s3.add_code(&mut vec![0x02,0x00,0x00,0x00,0x01,0x00,0x00,0x00]);
    // s3.end();
    s1.end();
    println!("{:?}",s1.variable_table);
}
