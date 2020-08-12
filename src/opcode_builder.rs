#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    NOP = 0x0000,
    IMMU16 = 0x1111,
    IMMU32 = 0x2222,
    IMMU64 = 0x3333,
    INC = 0x4444,
    MOV = 0x5555,
    ADD = 0x6666,
    EXTEND = 0x7777,
}
#[derive(Debug, Clone)]
pub enum OpCodeFormat{
    E0,
    E1(String),
    E2(String,String),
    E3(String,String,String),
    ED2(String,u16),
    ED3(String,u16,u16),
    D3(u16,u16,u16)
}
#[derive(Debug, Clone)]
pub struct OpCodeBuilder{
    pub data:Vec<(u16,OpCodeFormat)>,
}
use OpCodeFormat::*;
use crate::node::Function;
impl OpCodeBuilder{
    pub fn new()-> Self{
        OpCodeBuilder{data:vec![]}
    }
    pub fn get_and_clean(&mut self) -> Vec<(u16,OpCodeFormat)>{
        let ret = self.data.clone();
        self.data = vec![];
        ret
    }
    pub fn nop(&mut self){
        self.data.push((OpCode::NOP as u16,E0));
    }
    //                            u48
    pub fn extend(&mut self, data:u64){
        self.data.push((OpCode::EXTEND as u16,D3(data as u16, (data >> 16) as u16, (data >> 32) as u16)));
    }
    pub fn immu16(&mut self,variable: String, data:u16){
        self.data.push((OpCode::IMMU16 as u16,ED2(variable,data)));
    }
    pub fn immu32(&mut self,variable: String, data:u32){
        self.data.push((OpCode::IMMU32 as u16,ED3(variable,data as u16,(data>>16) as u16)));
    }
    pub fn immu64(&mut self,variable: String, data:u64){
        self.data.push((OpCode::IMMU64 as u16,ED3(variable,data as u16,(data>>16) as u16)));
        self.extend(data >> 32);
    }
    pub fn inc(&mut self,variable: String){
        self.data.push((OpCode::INC as u16,E1(variable)));
    }
    pub fn mov(&mut self,src: String, dst: String){
        self.data.push((OpCode::MOV as u16,E2(src,dst)));
    }
    pub fn add(&mut self,src1: String, src2:String, dst:String){
        self.data.push((OpCode::ADD as u16,E3(src1,src2,dst)));
    }
    pub fn gen_code(&self,function:&Function) -> Vec<u8>{
        let mut ret = vec![];
        for pc in 0..self.data.len(){
            let (ins,data) = &self.data[pc];
            let mut data = match data{
                E0 => vec![0x00,0x00,0x00,0x00,0x00,0x00],
                E1(dst) => {
                    let dst = find_suitable_register(function, dst.clone(), pc as u16);
                    vec![dst as u8, (dst >> 8) as u8, 0x00,0x00,0x00,0x00]
                },
                E2(src,dst) => {
                    let src = find_suitable_register(function, src.clone(), pc as u16);
                    let dst = find_suitable_register(function, dst.clone(), pc as u16);
                    vec![src as u8, (src >> 8) as u8,dst as u8, (dst >> 8) as u8,0x00,0x00]
                },
                E3(src1,src2,dst) => {
                    let src1 = find_suitable_register(function, src1.clone(), pc as u16);
                    let src2 = find_suitable_register(function, src2.clone(), pc as u16);
                    let dst = find_suitable_register(function, dst.clone(), pc as u16);
                    vec![src1 as u8, (src1 >> 8) as u8,src2 as u8, (src2 >> 8) as u8,dst as u8, (dst >> 8) as u8]
                },
                ED2(dst,d1) => {
                    let dst = find_suitable_register(function, dst.clone(), pc as u16);
                    vec![dst as u8, (dst >> 8) as u8, *d1 as u8,(d1 >> 8) as u8 ,0x00,0x00]
                },
                ED3(dst,d1,d2) => {
                    let dst = find_suitable_register(function, dst.clone(), pc as u16);
                    vec![dst as u8, (dst >> 8) as u8, *d1 as u8,(d1 >> 8) as u8 , *d2 as u8,(d2 >> 8) as u8 ]
                },
                D3(d1,d2,d3) => {
                    vec![*d1 as u8,(d1 >> 8) as u8 ,*d2 as u8,(d2 >> 8) as u8 ,*d3 as u8,(d3 >> 8) as u8 ]
                }
            };
            ret.push(*ins as u8);
            ret.push((ins >> 8) as u8);
            ret.append(&mut data);
        }
        ret
    }
}
fn find_suitable_register(function:&Function,variable:String,pc:u16) -> u16{
    for (r,vars) in &function.register_alloc_table{
        for v in vars{
            if v.name == variable{
                if v.end_pc > pc && v.start_pc <= pc {
                    return *r;
                }
            }
        }
    }
    panic!("ERROR! Could Not Find Variable {}",variable)
}

pub fn print_generated_code(code:&Vec<u8>){
    for i in 0..code.len()/8{
        print!("PC: {} | ",i);
        println!("{:02X?}",&code[i*8..i*8+8])
    }
}