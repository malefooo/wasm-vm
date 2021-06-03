use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::any::Any;
use crate::utils;

#[derive(Clone, Debug)]
pub enum ArgsEnum{
    BlockArgs(BlockArgs),
    IfArgs(IfArgs),
    BrTableArgs(BrTableArgs),
    MemArg(MemArg),
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    NONE,
}

impl ArgsEnum{

    pub fn get_i8(&self) -> i8{
        match self {
            ArgsEnum::I8(v) => {
                *v
            }
            v =>{panic!("{:?}",v)}
        }
    }

    pub fn get_u16(&self) -> u16{
        match self {
            ArgsEnum::U16(v) => {
                *v
            }
            v =>{panic!("{:?}",v)}
        }
    }

    pub fn get_i16(&self) -> i16{
        match self {
            ArgsEnum::I16(v) => {
                *v
            }
            v =>{panic!("{:?}",v)}
        }
    }

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

    pub fn get_u64(&self) -> u64{
        match self {
            ArgsEnum::U64(v) => {*v}
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

    pub fn get_f32(&self) -> f32{
        match self {
            ArgsEnum::F32(v)=>{*v}
            v =>{panic!("{:?}",v)}
        }
    }

    pub fn get_bool(&self) -> bool{
        match self {
            ArgsEnum::Bool(v)=>{*v}
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

    pub fn get_mem_args(&self) -> MemArg{
        match self {
            ArgsEnum::MemArg(v) => {v.clone()}
            v => {panic!("{:?}",v)}
        }
    }

    pub fn get_type(&self) -> &str{
        match self {
            ArgsEnum::BlockArgs(_) => {"BlockArgs"}
            ArgsEnum::IfArgs(_) => {"IfArgs"}
            ArgsEnum::BrTableArgs(_) => {"BrTableArgs"}
            ArgsEnum::MemArg(_) => {"MemArg"}
            ArgsEnum::Bool(_) => {"Bool"}
            ArgsEnum::U8(_) => {"U8"}
            ArgsEnum::U32(_) => {"U32"}
            ArgsEnum::U64(_) => {"U64"}
            ArgsEnum::I32(_) => {"I32"}
            ArgsEnum::I64(_) => {"I64"}
            ArgsEnum::F32(_) => {"F32"}
            ArgsEnum::F64(_) => {"F64"}
            ArgsEnum::I8(_) => {"I8"}
            ArgsEnum::I16(_) => {"I16"}
            ArgsEnum::U16(_) => {"U16"}
            ArgsEnum::NONE => {"None"}
        }
    }

}

impl PartialEq for ArgsEnum{
    fn eq(&self, other: &Self) -> bool {
        use ArgsEnum::*;

        return if self.get_type().eq(other.get_type()) {
            match self.get_type() {
                "Bool" => {
                    if self.get_bool().eq(&other.get_bool()) {true}
                    else{false}
                }
                "U8" => {
                    if self.get_u8().eq(&other.get_u8()) {true }
                    else{false}
                }
                "U32" => {
                    if self.get_u32().eq(&other.get_u32()) {true }
                    else{false}
                }
                "U64" => {
                    if self.get_u64().eq(&other.get_u64()) {true }
                    else{false}
                }
                "I32" => {
                    if self.get_i32().eq(&other.get_i32()) {true }
                    else{false}
                }
                "I64" => {
                    if self.get_i64().eq(&other.get_i64()) {true }
                    else{false}
                }
                "F32" =>{
                    if self.get_f32().eq(&other.get_f32()) {true }
                    else{false}
                }
                "F64" => {
                    if self.get_f64().eq(&other.get_f64()) {true }
                    else{false}
                }
                "I8" => {
                    if self.get_i8().eq(&other.get_i8()) {true }
                    else{false}
                }
                "I16" => {
                    if self.get_i16().eq(&other.get_i16()) {true }
                    else{false}
                }
                "U16" => {
                    if self.get_u16().eq(&other.get_u16()) {true }
                    else{false}
                }
                "None" =>{
                    true
                }
                _ => {false}
            }
        } else {
            false
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