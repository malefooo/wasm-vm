use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::fmt;

#[derive(Clone, Debug)]
pub enum ArgsEnum{
    BlockArgs(BlockArgs),
    IfArgs(IfArgs),
    BrTableArgs(BrTableArgs),
    MemArg(MemArg),
    U8(u8),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64)
}

impl ArgsEnum{
    pub fn get_i32(&self) -> i32{
        match self {
            ArgsEnum::I32(v) => {
                *v
            }
            v =>{panic!("{:?}",v)}
        }
    }
    
    pub fn get_u32(&self) -> u32{
        match self {
            ArgsEnum::U32(v) => {*v}
            v => {panic!("{:?}",v)}
        }
    }
    
    pub fn get_u8(&self) -> u8{
        match self {
            ArgsEnum::U8(v)=>{*v}
            v=> {panic!("{:?}",v)}
        }
    }
    
    pub fn get_i64(&self) -> i64{
        match self {
            ArgsEnum::I64(v)=>{*v}
            v =>{panic!("{:?}",v)}
        }
    }
    
    pub fn get_f64(&self) -> f64{
        match self {
            ArgsEnum::F64(v)=>{*v}
            v =>{panic!("{:?}",v)}
        }
    }
    
    pub fn get_block_args(&self) -> BlockArgs{
        match self {
            ArgsEnum::BlockArgs(v)=>{v.clone()}
            v =>{panic!("{:?}",v)}
        }
    }
    
    pub fn get_if_args(&self) -> IfArgs{
        match self {
            ArgsEnum::IfArgs(v) => {v.clone()}
            v => {panic!("{:?}",v)}
        }
    }
}

pub type Expr = Vec<Instruction>;

#[derive(Clone, Debug)]
pub struct Instruction  {
    pub opcode:Option<u8>,
    pub args:Option<ArgsEnum>,
}
#[derive(Clone, Debug)]
pub struct BlockArgs{
    pub bt:Option<i32>,
    pub instrs:Option<Vec<Instruction>>
}
#[derive(Clone, Debug)]
pub struct IfArgs{
    pub bt:Option<i32>,
    pub instrs1:Option<Vec<Instruction>>,
    pub instrs2:Option<Vec<Instruction>>,
}
#[derive(Clone, Debug)]
pub struct BrTableArgs{
    pub labels:Option<Vec<u32>>,
    pub default:Option<u32>,
}
#[derive(Clone, Debug)]
pub struct MemArg{
    pub align:Option<u32>,
    pub offset:Option<u32>,
}

impl Instruction{

    pub fn new(opcode:u8,data:ArgsEnum) -> Instruction{
        println!("{:?}",data.get_block_args());
        Instruction{
            opcode: Some(opcode),
            args: Some(data),
        }

    }

    pub fn get_op_name(&self) -> &str{
        match crate::binary::opcodes::OPCODE_MAP.get(){
            None => {panic!("get opcode map none")}
            Some(map) => {
                match self.opcode {
                    None => {panic!("the Instruction opcode is none")}
                    Some(o) => {
                        let v = map.get(&o).unwrap();
                        v
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::binary::instruction::{ArgsEnum, BlockArgs};

    #[test]
    fn test(){
        use crate::binary;
        binary::init();
        let i = binary::instruction::Instruction::new(0x02,ArgsEnum::BlockArgs(BlockArgs{ bt: None, instrs: None }));
        println!("{:?}",i.get_op_name());
    }
}