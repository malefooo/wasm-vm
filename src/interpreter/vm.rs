use crate::{interpreter::operand,binary};


use crate::{binary::instruction::ArgsEnum,binary::opcodes,utils};
use once_cell::sync::OnceCell;
use std::os::unix::raw::uid_t;
use std::any::type_name;
use bitintr::{Lzcnt, Tzcnt, Popcnt};
use std::ops::Neg;

type InstrFn = fn(vm:&mut Vm,args:ArgsEnum);
pub static OPCODE_MAP:OnceCell<Vec<Option<InstrFn>>> = OnceCell::new();

pub fn init(){
    let mut v:Vec<Option<InstrFn>> = Vec::new();
    v.resize(256,None);
    v.insert(opcodes::Call as usize, Some(|vm:&mut Vm, args:ArgsEnum|{}));
    v.insert(opcodes::Drop as usize, Some(|vm: &mut Vm, args:ArgsEnum|{vm.drop()}));
    v.insert(opcodes::Select as usize, Some(|vm:&mut Vm, args:ArgsEnum|{vm.select()}));
    v.insert(opcodes::I32Const as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_const(args.get_i32())}));
    v.insert(opcodes::I64Const as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_const(args.get_i64())}));
    v.insert(opcodes::F32Const as usize, Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_const(args.get_f32())}));
    v.insert(opcodes::F64Const as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_const(args.get_f64())}));
    v.insert(opcodes::I32Eqz as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_eqz()}));
    v.insert(opcodes::I32Eq as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_eq()}));
    v.insert(opcodes::I32Ne as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_ne()}));
    v.insert(opcodes::I32LtS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_lts()}));
    v.insert(opcodes::I32LtU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_ltu()}));
    v.insert(opcodes::I32GtS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_gts()}));
    v.insert(opcodes::I32GtU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_gtu()}));
    v.insert(opcodes::I32LeS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_les()}));
    v.insert(opcodes::I32LeU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_leu()}));
    v.insert(opcodes::I32GeS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_ges()}));
    v.insert(opcodes::I32GeU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_geu()}));
    v.insert(opcodes::I64Eqz as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_eqz()}));
    v.insert(opcodes::I64Eq as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_eq()}));
    v.insert(opcodes::I64Ne as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_ne()}));
    v.insert(opcodes::I64LtS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_lts()}));
    v.insert(opcodes::I64LtU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_ltu()}));
    v.insert(opcodes::I64GtS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_gts()}));
    v.insert(opcodes::I64GtU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_gtu()}));
    v.insert(opcodes::I64LeS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_les()}));
    v.insert(opcodes::I64LeU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_leu()}));
    v.insert(opcodes::I64GeS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_ges()}));
    v.insert(opcodes::I64GeU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_geu()}));
    v.insert(opcodes::F32Eq as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_eq()}));
    v.insert(opcodes::F32Ne as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_ne()}));
    v.insert(opcodes::F32Lt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_lt()}));
    v.insert(opcodes::F32Gt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_gt()}));
    v.insert(opcodes::F32Le as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_le()}));
    v.insert(opcodes::F32Ge as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_ge()}));
    v.insert(opcodes::F64Eq as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_eq()}));
    v.insert(opcodes::F64Ne as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_ne()}));
    v.insert(opcodes::F64Lt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_lt()}));
    v.insert(opcodes::F64Gt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_gt()}));
    v.insert(opcodes::F64Le as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_le()}));
    v.insert(opcodes::F64Ge as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_ge()}));
    v.insert(opcodes::I32Clz as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_clz()}));
    v.insert(opcodes::I32Ctz as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_ctz()}));
    v.insert(opcodes::I32PopCnt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_popcnt()}));
    v.insert(opcodes::I32Add as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_add()}));
    v.insert(opcodes::I32Sub as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_sub()}));
    v.insert(opcodes::I32Mul as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_mul()}));
    v.insert(opcodes::I32DivS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_divs()}));
    v.insert(opcodes::I32DivU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_divu()}));
    v.insert(opcodes::I32RemS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_rems()}));
    v.insert(opcodes::I32RemU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_remu()}));
    v.insert(opcodes::I32And as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_and()}));
    v.insert(opcodes::I32Or as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_or()}));
    v.insert(opcodes::I32Xor as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_xor()}));
    v.insert(opcodes::I32Shl as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_shl()}));
    v.insert(opcodes::I32ShrS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_shrs()}));
    v.insert(opcodes::I32ShrU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_shru()}));
    v.insert(opcodes::I32Rotl as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_rotl()}));
    v.insert(opcodes::I32Rotr as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_rotr()}));
    v.insert(opcodes::I64Clz as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_clz()}));
    v.insert(opcodes::I64Ctz as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_ctz()}));
    v.insert(opcodes::I64PopCnt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_popcnt()}));
    v.insert(opcodes::I64Add as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_add()}));
    v.insert(opcodes::I64Sub as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_sub()}));
    v.insert(opcodes::I64Mul as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_mul()}));
    v.insert(opcodes::I64DivS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_divs()}));
    v.insert(opcodes::I64DivU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_divu()}));
    v.insert(opcodes::I64RemS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_rems()}));
    v.insert(opcodes::I64RemU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_remu()}));
    v.insert(opcodes::I64And as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_and()}));
    v.insert(opcodes::I64Or as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_or()}));
    v.insert(opcodes::I64Xor as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_xor()}));
    v.insert(opcodes::I64Shl as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_shl()}));
    v.insert(opcodes::I64ShrS as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_shrs()}));
    v.insert(opcodes::I64ShrU as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_shru()}));
    v.insert(opcodes::I64Rotl as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_rotl()}));
    v.insert(opcodes::I64Rotr as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_rotr()}));
    v.insert(opcodes::F32Abs as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_abs()}));
    v.insert(opcodes::F32Neg as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_neg()}));
    v.insert(opcodes::F32Ceil as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_ceil()}));
    v.insert(opcodes::F32Floor as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_floor()}));
    v.insert(opcodes::F32Trunc as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_trunc()}));
    v.insert(opcodes::F32Nearest as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_nearest()}));
    v.insert(opcodes::F32Sqrt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_sqrt()}));
    v.insert(opcodes::F32Add as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_add()}));
    v.insert(opcodes::F32Sub as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_sub()}));
    v.insert(opcodes::F32Mul as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_mul()}));
    v.insert(opcodes::F32Div as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_div()}));
    v.insert(opcodes::F32Min as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_min()}));
    v.insert(opcodes::F32Max as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_max()}));
    v.insert(opcodes::F32CopySign as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_copy_sign()}));
    v.insert(opcodes::F32Abs as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_abs()}));
    v.insert(opcodes::F32Neg as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_neg()}));
    v.insert(opcodes::F32Ceil as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_ceil()}));
    v.insert(opcodes::F32Floor as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_floor()}));
    v.insert(opcodes::F32Trunc as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_trunc()}));
    v.insert(opcodes::F32Nearest as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_nearest()}));
    v.insert(opcodes::F32Sqrt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_sqrt()}));
    v.insert(opcodes::F32Add as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_add()}));
    v.insert(opcodes::F32Sub as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_sub()}));
    v.insert(opcodes::F32Mul as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_mul()}));
    v.insert(opcodes::F32Div as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_div()}));
    v.insert(opcodes::F32Min as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_min()}));
    v.insert(opcodes::F32Max as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_max()}));
    v.insert(opcodes::F32CopySign as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_copy_sign()}));
    v.insert(opcodes::F64Abs as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_abs()}));
    v.insert(opcodes::F64Neg as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_neg()}));
    v.insert(opcodes::F64Ceil as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_ceil()}));
    v.insert(opcodes::F64Floor as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_floor()}));
    v.insert(opcodes::F64Trunc as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_trunc()}));
    v.insert(opcodes::F64Nearest as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_nearest()}));
    v.insert(opcodes::F64Sqrt as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_sqrt()}));
    v.insert(opcodes::F64Add as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_add()}));
    v.insert(opcodes::F64Sub as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_sub()}));
    v.insert(opcodes::F64Mul as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_mul()}));
    v.insert(opcodes::F64Div as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_div()}));
    v.insert(opcodes::F64Min as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_min()}));
    v.insert(opcodes::F64Max as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_max()}));
    v.insert(opcodes::F64CopySign as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_copy_sign()}));
    v.insert(opcodes::I32WrapI64 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_warp_i64()}));
    v.insert(opcodes::I32TruncF32S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_trunc_f32_s()}));
    v.insert(opcodes::I32TruncF32U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_trunc_f32_u()}));
    v.insert(opcodes::I32TruncF64S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_trunc_f64_s()}));
    v.insert(opcodes::I32TruncF64U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_trunc_f64_u()}));
    v.insert(opcodes::I64ExtendI32S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_extend_i32_s()}));
    v.insert(opcodes::I64ExtendI32U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_extend_i32_u()}));
    v.insert(opcodes::I64TruncF32S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_trunc_f32_s()}));
    v.insert(opcodes::I64TruncF32U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_trunc_f32_u()}));
    v.insert(opcodes::I64TruncF64S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_trunc_f64_s()}));
    v.insert(opcodes::I64TruncF64U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_trunc_f64_u()}));
    v.insert(opcodes::F32ConvertI32S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_convert_i32_s()}));
    v.insert(opcodes::F32ConvertI32U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_convert_i32_u()}));
    v.insert(opcodes::F32ConvertI64S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_convert_i64_s()}));
    v.insert(opcodes::F32ConvertI64U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_convert_i64_u()}));
    v.insert(opcodes::F32DemoteF64 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f32_demote_f64()}));
    v.insert(opcodes::F64ConvertI32S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_convert_i32_s()}));
    v.insert(opcodes::F64ConvertI32U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_convert_i32_u()}));
    v.insert(opcodes::F64ConvertI64S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_convert_i64_s()}));
    v.insert(opcodes::F64ConvertI64U as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_convert_i64_u()}));
    v.insert(opcodes::F64PromoteF32 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.f64_promote_f32()}));
    v.insert(opcodes::I32ReinterpretF32 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{}));
    v.insert(opcodes::I64ReinterpretF64 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{}));
    v.insert(opcodes::F32ReinterpretI32 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{}));
    v.insert(opcodes::F64ReinterpretI64 as usize,Some(|vm:&mut Vm, args:ArgsEnum|{}));
    v.insert(opcodes::I32Extend8S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_extend_8_s()}));
    v.insert(opcodes::I32Extend16S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i32_extend_16_s()}));
    v.insert(opcodes::I64Extend8S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_extend_8_s()}));
    v.insert(opcodes::I64Extend16S as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.i64_extend_16_s()}));
    v.insert(opcodes::TruncSat as usize,Some(|vm:&mut Vm, args:ArgsEnum|{vm.trunc_sat(args.get_u8())}));

    OPCODE_MAP.set(v);
}

#[derive(Debug,Clone)]
pub struct Vm {
    operand_stack:operand::OperandStack,
    module:binary::module::Module,
}

/// i32
impl Vm {
    //比较指令
    pub fn i32_eqz(&mut self){
        let v = self.operand_stack.pop_s32().unwrap();
        if v == 0 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_eq(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 == v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_ne(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 != v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_lts(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 < v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_ltu(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();

        if v1 < v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_gts(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 > v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_gtu(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();

        if v1 > v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_les(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 <= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_leu(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();

        if v1 <= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_ges(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 >= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i32_geu(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();

        if v1 >= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    // 一元算术
    pub fn i32_clz(&mut self){
        let v = self.operand_stack.pop_u32().unwrap();
        let v = v.lzcnt();
        self.operand_stack.push_u32(v);
    }

    pub fn i32_ctz(&mut self){
        let v = self.operand_stack.pop_u32().unwrap();
        let v = v.tzcnt();
        self.operand_stack.push_u32(v);
    }

    pub fn i32_popcnt(&mut self){
        let v = self.operand_stack.pop_u32().unwrap();
        let v = v.popcnt();
        self.operand_stack.push_u32(v);
    }

    // 二元算术
    pub fn i32_add(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1+v2);
    }

    pub fn i32_sub(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1-v2);
    }

    pub fn i32_mul(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1*v2);
    }

    pub fn i32_divs(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        if v1 == i32::MIN && v2 == -1{
            panic!("errIntOverflow")
        }

        if v2 == 0 {
            panic!("Dividend is 0")
        }

        self.operand_stack.push_s32(v1/v2);
    }

    pub fn i32_divu(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();

        self.operand_stack.push_u32(v1/v2);
    }

    pub fn i32_rems(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();

        self.operand_stack.push_s32(v1%v2);
    }

    pub fn i32_remu(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();

        self.operand_stack.push_u32(v1%v2);
    }

    pub fn i32_and(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1&v2);
    }

    pub fn i32_or(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1|v2);
    }

    pub fn i32_xor(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1^v2);
    }

    pub fn i32_shl(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1 << (v2%32));
    }

    pub fn i32_shrs(&mut self){
        let v2 = self.operand_stack.pop_s32().unwrap();
        let v1 = self.operand_stack.pop_s32().unwrap();
        self.operand_stack.push_s32(v1 >> (v2%32));
    }

    pub fn i32_shru(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1 >> (v2%32));
    }

    pub fn i32_rotl(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1.rotate_left(v2));
    }

    pub fn i32_rotr(&mut self){
        let v2 = self.operand_stack.pop_u32().unwrap();
        let v1 = self.operand_stack.pop_u32().unwrap();
        self.operand_stack.push_u32(v1.rotate_right(v2));
    }


    // 类型转换指令
}

/// i64
impl Vm {
    //0x46
    pub fn i64_eqz(&mut self){
        let v = self.operand_stack.pop_s64().unwrap();
        if v == 0 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_eq(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        if v1==v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_ne(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();
        if v1 != v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_lts(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        if v1 < v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_ltu(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();

        if v1 < v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_gts(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        if v1 > v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_gtu(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();

        if v1 > v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_les(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        if v1 <= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_leu(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();

        if v1 <= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_ges(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        if v1 >= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn i64_geu(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();

        if v1 >= v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }


    // 一元算术
    pub fn i64_clz(&mut self){
        let v = self.operand_stack.pop_u64().unwrap();
        let v = v.lzcnt();
        self.operand_stack.push_u64(v);
    }

    pub fn i64_ctz(&mut self){
        let v = self.operand_stack.pop_u64().unwrap();
        let v = v.tzcnt();
        self.operand_stack.push_u64(v);
    }

    pub fn i64_popcnt(&mut self){
        let v = self.operand_stack.pop_u64().unwrap();
        let v = v.popcnt();
        self.operand_stack.push_u64(v);
    }

    // 二元算术
    pub fn i64_add(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1+v2);
    }

    pub fn i64_sub(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1-v2);
    }

    pub fn i64_mul(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1*v2);
    }

    pub fn i64_divs(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        if v1 == i64::MIN && v2 == -1{
            panic!("errIntOverflow")
        }

        if v2 == 0 {
            panic!("Dividend is 0")
        }

        self.operand_stack.push_s64(v1/v2);
    }

    pub fn i64_divu(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();

        self.operand_stack.push_u64(v1/v2);
    }

    pub fn i64_rems(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();

        self.operand_stack.push_s64(v1%v2);
    }

    pub fn i64_remu(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();

        self.operand_stack.push_u64(v1%v2);
    }

    pub fn i64_and(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1&v2);
    }

    pub fn i64_or(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1|v2);
    }

    pub fn i64_xor(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1^v2);
    }

    pub fn i64_shl(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1 << (v2%32));
    }

    pub fn i64_shrs(&mut self){
        let v2 = self.operand_stack.pop_s64().unwrap();
        let v1 = self.operand_stack.pop_s64().unwrap();
        self.operand_stack.push_s64(v1 >> (v2%32));
    }

    pub fn i64_shru(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1 >> (v2%32));
    }

    pub fn i64_rotl(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1.rotate_left(v2 as u32));
    }

    pub fn i64_rotr(&mut self){
        let v2 = self.operand_stack.pop_u64().unwrap();
        let v1 = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u64(v1.rotate_right(v2 as u32));
    }
}

/// f32
impl Vm {
    pub fn f32_eq(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        if v1==v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f32_ne(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        if v1!=v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f32_lt(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        if v1<v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f32_gt(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        if v1>v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f32_le(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        if v1<=v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f32_ge(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        if v1>=v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }


    // 一元算术
    pub fn f32_abs(&mut self){
        let v= self.operand_stack.pop_f32().unwrap();
        let v = v.abs();
        self.operand_stack.push_f32(v);
    }

    pub fn f32_neg(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.neg();
        self.operand_stack.push_f32(v);
    }

    pub fn f32_ceil(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.ceil();
        self.operand_stack.push_f32(v);
    }

    pub fn f32_floor(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.floor();
        self.operand_stack.push_f32(v);
    }

    pub fn f32_trunc(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.trunc();
        self.operand_stack.push_f32(v);
    }

    pub fn f32_nearest(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.round();
        self.operand_stack.push_f32(v);
    }

    pub fn f32_sqrt(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.sqrt();
        self.operand_stack.push_f32(v);
    }


    // 二元
    pub fn f32_add(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        self.operand_stack.push_f32(v1 + v2);
    }

    pub fn f32_sub(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        self.operand_stack.push_f32(v1-v2);
    }

    pub fn f32_mul(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        self.operand_stack.push_f32(v1 * v2);
    }

    pub fn f32_div(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        self.operand_stack.push_f32(v1 / v2);
    }

    pub fn f32_min(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        let b1 = v1.is_nan();
        let b2 = v2.is_nan();

        if b1 && !b2 {
            self.operand_stack.push_f32(v1);
            return
        } else if !b1 && b2 {
            self.operand_stack.push_f32(v2);
            return
        }

        if v1 > v2 {
            self.operand_stack.push_f32(v2);
        } else {
            self.operand_stack.push_f32(v1);
        }


    }

    pub fn f32_max(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        let b1 = v1.is_nan();
        let b2 = v2.is_nan();

        if b1 && !b2 {
            self.operand_stack.push_f32(v1);
            return
        } else if !b1 && b2 {
            self.operand_stack.push_f32(v2);
            return
        }

        if v1 > v2 {
            self.operand_stack.push_f32(v1)
        } else {
            self.operand_stack.push_f32(v2)
        }

    }

    pub fn f32_copy_sign(&mut self){
        let v2 = self.operand_stack.pop_f32().unwrap();
        let v1 = self.operand_stack.pop_f32().unwrap();

        self.operand_stack.push_f32(v1.copysign(v2));
    }
}

/// f64
impl Vm {
    pub fn f64_eq(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        if v1==v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f64_ne(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        if v1!=v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f64_lt(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        if v1<v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f64_gt(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        if v1>v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f64_le(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        if v1<=v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }

    pub fn f64_ge(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        if v1>=v2 {
            self.operand_stack.push_bool(1);
        } else {
            self.operand_stack.push_bool(0);
        }
    }


    // 一元算术
    pub fn f64_abs(&mut self){
        let v= self.operand_stack.pop_f64().unwrap();
        let v = v.abs();
        self.operand_stack.push_f64(v);
    }

    pub fn f64_neg(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.neg();
        self.operand_stack.push_f64(v);
    }

    pub fn f64_ceil(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.ceil();
        self.operand_stack.push_f64(v);
    }

    pub fn f64_floor(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.floor();
        self.operand_stack.push_f64(v);
    }

    pub fn f64_trunc(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.trunc();
        self.operand_stack.push_f64(v);
    }

    pub fn f64_nearest(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.round();
        self.operand_stack.push_f64(v);
    }

    pub fn f64_sqrt(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.sqrt();
        self.operand_stack.push_f64(v);
    }

    //二元
    pub fn f64_add(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        self.operand_stack.push_f64(v1 + v2);
    }

    pub fn f64_sub(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        self.operand_stack.push_f64(v1 - v2);
    }

    pub fn f64_mul(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        self.operand_stack.push_f64(v1*v2);
    }

    pub fn f64_div(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        self.operand_stack.push_f64(v1/v2);
    }

    pub fn f64_min(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        let b1 = v1.is_nan();
        let b2 = v2.is_nan();

        if b1 && !b2 {
            self.operand_stack.push_f64(v1);
            return
        } else if !b1 && b2 {
            self.operand_stack.push_f64(v2);
            return
        }

        if v1 > v2 {
            self.operand_stack.push_f64(v2);
        } else {
            self.operand_stack.push_f64(v1);
        }
    }

    pub fn f64_max(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        let b1 = v1.is_nan();
        let b2 = v2.is_nan();

        if b1 && !b2 {
            self.operand_stack.push_f64(v1);
            return
        } else if !b1 && b2 {
            self.operand_stack.push_f64(v2);
            return
        }
        if v1 > v2 {
            self.operand_stack.push_f64(v1);
        } else {
            self.operand_stack.push_f64(v2)
        }
    }

    pub fn f64_copy_sign(&mut self){
        let v2 = self.operand_stack.pop_f64().unwrap();
        let v1 = self.operand_stack.pop_f64().unwrap();

        self.operand_stack.push_f64(v1.copysign(v2));
    }
}



impl Vm {

    pub fn new(var1:operand::OperandStack,var2:binary::module::Module) -> Vm{
        Vm{ operand_stack: var1, module: var2 }
    }

    // pub fn exec_code(&mut self, idx:usize){
    //     self.module.code_sec.clone()
    //         .and_then(|v|v.get(idx).cloned()).or_else(||{println!("vec code none");None})
    //         .and_then(|code|code.expr).or_else(||{println!("code expr none");None})
    //         .and_then(|expr|{
    //             for x in expr {
    //                 self.exec_instr(x);
    //             }
    //             Some(())
    //         }).expect("exec code none");
    // }

    // pub fn exec_instr(&mut self, instr:binary::instruction::Instruction){
    //     OPCODE_MAP.get()
    //         .and_then(|v| v.get(instr.opcode.unwrap() as usize))
    //         .and_then(|f| {f(self,instr.args.unwrap());Some(())});
    // }

    //0x1A
    pub fn drop(&mut self){
        self.operand_stack.pop();
    }

    //0x1B
    pub fn select(&mut self){
        let v3 = self.operand_stack.pop_bool();
        let v2 = self.operand_stack.pop().unwrap();
        let v1 = self.operand_stack.pop().unwrap();

        if v1.eq(&v2) {
            v3.and_then(|b|{
                if b {
                    self.operand_stack.push(v1);
                } else {
                    self.operand_stack.push(v2);
                };
               Some(())
            });
        } else {
            panic!("Data format does not match")
        }
    }

    //0x41
    pub fn i32_const(&mut self,val:i32){
        self.operand_stack.push(ArgsEnum::I32(val))
    }

    //0x42
    pub fn i64_const(&mut self,val:i64){
        self.operand_stack.push(ArgsEnum::I64(val))
    }

    //0x43
    pub fn f32_const(&mut self,val:f32){
        self.operand_stack.push(ArgsEnum::F32(val))
    }

    //0x44
    pub fn f64_const(&mut self,val:f64){
        self.operand_stack.push(ArgsEnum::F64(val))
    }

}

impl Vm{
    pub fn i32_warp_i64(&mut self){
        let v = self.operand_stack.pop_u64().unwrap();
        self.operand_stack.push_u32(v as u32);
    }

    pub fn i32_trunc_f32_s(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.trunc() as i32;
        self.operand_stack.push_s32(v);
    }

    pub fn i32_trunc_f32_u(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.trunc() as u32;
        self.operand_stack.push_u32(v);
    }

    pub fn i32_trunc_f64_s(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.trunc() as i32;
        if v > i32::MAX || v < i32::MIN {
            panic!("errIntOverflow")
        }

        self.operand_stack.push_s32(v);
    }

    pub fn i32_trunc_f64_u(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.trunc() as u32;
        if v > u32::MAX || v < 0 {
            panic!("errIntOverflow")
        }
        self.operand_stack.push_u32(v);
    }

    pub fn i64_extend_i32_s(&mut self){
        let v = self.operand_stack.pop_s32().unwrap();
        let v = v as i64;
        self.operand_stack.push_s64(v);
    }

    pub fn i64_extend_i32_u(&mut self){
        let v = self.operand_stack.pop_u32().unwrap();
        let v = v as u64;
        self.operand_stack.push_u64(v);
    }


    pub fn i64_trunc_f32_s(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v.trunc() as i64;
        if v > i64::MAX || v < i64::MIN {
            panic!("errIntOverflow")
        }
        self.operand_stack.push_s64(v);
    }

    pub fn i64_trunc_f32_u(&mut self){
        let v= self.operand_stack.pop_f32().unwrap();
        let v = v.trunc() as u64;
        if v > u64::MAX || v < 0 {
            panic!("errIntOverflow")
        }
        self.operand_stack.push_u64(v);
    }

    pub fn i64_trunc_f64_s(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v.trunc() as i64;
        if v > i64::MAX || v < i64::MIN {
            panic!("errIntOverflow")
        }
        self.operand_stack.push_s64(v);
    }

    pub fn i64_trunc_f64_u(&mut self){
        let v= self.operand_stack.pop_f64().unwrap();
        let v = v.trunc() as u64;
        if v > u64::MAX || v < 0 {
            panic!("errIntOverflow")
        }
        self.operand_stack.push_u64(v);
    }

    pub fn f32_convert_i32_s(&mut self){
        let v = self.operand_stack.pop_s32().unwrap();
        let v = v as f32;
        self.operand_stack.push_f32(v);
    }

    pub fn f32_convert_i32_u(&mut self){
        let v = self.operand_stack.pop_u32().unwrap();
        let v = v as f32;
        self.operand_stack.push_f32(v);
    }

    pub fn f32_convert_i64_s(&mut self){
        let v = self.operand_stack.pop_s64().unwrap();
        let v = v as f32;
        self.operand_stack.push_f32(v);
    }

    pub fn f32_convert_i64_u(&mut self){
        let v= self.operand_stack.pop_u64().unwrap();
        let v = v as f32;
        self.operand_stack.push_f32(v);
    }

    pub fn f32_demote_f64(&mut self){
        let v = self.operand_stack.pop_f64().unwrap();
        let v = v as f32;
        self.operand_stack.push_f32(v);
    }

    pub fn f64_convert_i32_s(&mut self){
        let v = self.operand_stack.pop_s32().unwrap();
        let v = v as f64;
        self.operand_stack.push_f64(v);
    }

    pub fn f64_convert_i32_u(&mut self){
        let v = self.operand_stack.pop_u32().unwrap();
        let v = v as f64;
        self.operand_stack.push_f64(v);
    }

    pub fn f64_convert_i64_s(&mut self){
        let v = self.operand_stack.pop_s64().unwrap();
        let v = v as f64;
        self.operand_stack.push_f64(v);
    }

    pub fn f64_convert_i64_u(&mut self){
        let v = self.operand_stack.pop_u64().unwrap();
        let v = v as f64;
        self.operand_stack.push_f64(v);
    }

    pub fn f64_promote_f32(&mut self){
        let v = self.operand_stack.pop_f32().unwrap();
        let v = v as f64;
        self.operand_stack.push_f64(v);
    }

    pub fn i32_extend_8_s(&mut self){
        let v = self.operand_stack.pop_s32().unwrap();
        let v = v as i8;
        self.operand_stack.push_i8(v);
    }

    pub fn i32_extend_16_s(&mut self){
        let v = self.operand_stack.pop_s32().unwrap();
        let v = v as i16;
        self.operand_stack.push_i16(v);
    }

    pub fn i64_extend_8_s(&mut self){
        let v = self.operand_stack.pop_s64().unwrap();
        let v = v as i8;
        self.operand_stack.push_i8(v);
    }

    pub fn i64_extend_16_s(&mut self){
        let v = self.operand_stack.pop_s64().unwrap();
        let v = v as i16;
        self.operand_stack.push_i16(v);
    }

    pub fn i64_extend_32_s(&mut self){
        let v = self.operand_stack.pop_s64().unwrap();
        let v = v as i32;
        self.operand_stack.push_s32(v);
    }

    pub fn trunc_sat(&mut self,val:u8){
        match val {
            0 => {
                let v = self.operand_stack.pop_f32().unwrap();
                let result = trunc_sat_s(v as f64,32);
                self.operand_stack.push_s32(result as i32);
            }
            1 => {
                let v = self.operand_stack.pop_f32().unwrap();
                let result = trunc_sat_u(v as f64, 32);
                self.operand_stack.push_u32(result as u32);
            }
            2 => {
                let v = self.operand_stack.pop_f64().unwrap();
                let result = trunc_sat_s(v,32);
                self.operand_stack.push_s32(result as i32);
            }
            3 => {
                let v = self.operand_stack.pop_f64().unwrap();
                let result = trunc_sat_u(v,32);
                self.operand_stack.push_u32(result as u32);
            }
            4 => {
                let v = self.operand_stack.pop_f32().unwrap();
                let result = trunc_sat_s(v as f64,64);
                self.operand_stack.push_s64(result);
            }
            5 => {
                let v = self.operand_stack.pop_f32().unwrap();
                let result = trunc_sat_u(v as f64,64);
                self.operand_stack.push_u64(result)
            }
            6 => {
                let v = self.operand_stack.pop_f64().unwrap();
                let result = trunc_sat_s(v,64);
                self.operand_stack.push_s64(result);
            }
            7 => {
                let v = self.operand_stack.pop_f64().unwrap();
                let result = trunc_sat_u(v,64);
                self.operand_stack.push_u64(result);
            }
            _ => {
                panic!("unreachable")
            }
        }
    }


}

pub fn trunc_sat_u(z:f64,n:usize) -> u64{
    if z.is_nan() {
        return 0;
    }
    let max = (1_u64 << n) -1;
    if z.is_infinite() {
        return if z > 0.0 {
            max
        } else {
            0
        }
    }

    let x = z.trunc();
    return if x < 0.0 {
        0
    } else if x >= (max as f64) {
        max
    } else {
        x as u64
    }
}

pub fn trunc_sat_s(z:f64,n:usize) -> i64{
    if z.is_nan() {
        return 0;
    }

    let min = -(1_i64 << (n-1));
    let max = (1_i64 << (n-1)) -1;

    if z.is_infinite() {
        return if z > 0.0 {
            max
        } else {
            min
        }
    }

    let x = z.trunc();
    return if x < (min as f64) {
        min
    } else if x>= (max as f64){
        max
    } else {
        x as i64
    }
}

#[cfg(test)]
mod test{
    use crate::{binary,interpreter};
    use crate::binary::instruction::ArgsEnum;
    use std::sync::atomic::Ordering::AcqRel;
    use crate::binary::instruction::ArgsEnum::{F32, F64, I64, I32, U32, U64};
    use crate::interpreter::vm::OPCODE_MAP;

    #[test]
    pub fn test1(){

    }

    #[test]
    pub fn test2(){
        let m = binary::module::Module::new();
        let stack = interpreter::operand::new();
        let mut vm = interpreter::vm::Vm{
            operand_stack: stack,
            module: m
        };

        vm.i32_const(100_i32);
        vm.i64_const(200_i64);
        vm.f32_const(1.5_f32);
        vm.f64_const(2.5_f64);

        assert_eq!(Some(2.5_f64),vm.operand_stack.pop_f64());
        assert_eq!(Some(1.5_f32),vm.operand_stack.pop_f32());
        assert_eq!(Some(200_i64),vm.operand_stack.pop_s64());
        assert_eq!(Some(100_i32),vm.operand_stack.pop_s32());
    }



    pub fn none_args(vm: &mut interpreter::vm::Vm, var1:ArgsEnum, var2:ArgsEnum, op_code:u8) -> binary::instruction::ArgsEnum{
        vm.operand_stack.push(var1);
        vm.operand_stack.push(var2);
        interpreter::vm::OPCODE_MAP.get()
            .and_then(|v|v.get(op_code as usize).clone())
            .or_else(||{println!("get op none");None})
            .and_then(|o|{
                o.unwrap()(vm,ArgsEnum::NONE);
                let r = vm.operand_stack.pop().unwrap();
                Some(r)
            }).or_else(||{println!("exec none");None})
            .unwrap()
    }

    pub fn none_args_2(vm: &mut interpreter::vm::Vm, var1:ArgsEnum, op_code:u8) -> binary::instruction::ArgsEnum{
        vm.operand_stack.push(var1);

        interpreter::vm::OPCODE_MAP.get()
            .and_then(|v|v.get(op_code as usize).clone())
            .or_else(||{println!("get op none");None})
            .and_then(|o|{
                o.unwrap()(vm,ArgsEnum::NONE);
                let r = vm.operand_stack.pop().unwrap();
                Some(r)
            }).or_else(||{println!("exec none");None})
            .unwrap()
    }

    #[test]
    pub fn test3(){
        use crate::binary::opcodes;
        let m = binary::module::Module::new();
        let stack = interpreter::operand::new();
        let mut vm = interpreter::vm::Vm{
            operand_stack: stack,
            module: m
        };

        //初始话操作数组
        interpreter::vm::init();

        //i32eq
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(1),opcodes::I32Eq),ArgsEnum::Bool(true));
        //i32ne
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(1),opcodes::I32Ne),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(-1),opcodes::I32Ne),ArgsEnum::Bool(false));
        //i32lts
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-1),ArgsEnum::I32(1),opcodes::I32LtS),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(-1),opcodes::I32LtS),ArgsEnum::Bool(false));
        //i32ltu
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(2),ArgsEnum::U32(1),opcodes::I32LtU),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(1),ArgsEnum::U32(1),opcodes::I32LtU),ArgsEnum::Bool(true));
        //i32gts
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-1),ArgsEnum::I32(1),opcodes::I32GtS),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(-1),opcodes::I32GtS),ArgsEnum::Bool(true));
        //i32gtu
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(1),ArgsEnum::U32(2),opcodes::I32GtU),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(2),ArgsEnum::U32(1),opcodes::I32GtU),ArgsEnum::Bool(true));
        //i32les
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-1),ArgsEnum::I32(1),opcodes::I32LeS),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(-1),opcodes::I32LeS),ArgsEnum::Bool(false));
        //i32leu
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(1),ArgsEnum::U32(2),opcodes::I32LeU),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(2),ArgsEnum::U32(1),opcodes::I32LeU),ArgsEnum::Bool(false));
        //i32ges
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-1),ArgsEnum::I32(1),opcodes::I32GeS),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(1),ArgsEnum::I32(-1),opcodes::I32GeS),ArgsEnum::Bool(true));
        //i32geu
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(2),ArgsEnum::U32(1),opcodes::I32GeU),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(1),ArgsEnum::U32(2),opcodes::I32GeU),ArgsEnum::Bool(false));

        //i64eq
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(1),opcodes::I64Eq),ArgsEnum::Bool(true));
        //i64ne
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(1),opcodes::I64Ne),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(-1),opcodes::I64Ne),ArgsEnum::Bool(false));
        //i64lts
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-1),ArgsEnum::I64(1),opcodes::I64LtS),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(-1),opcodes::I64LtS),ArgsEnum::Bool(false));
        //i64ltu
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(2),ArgsEnum::U64(1),opcodes::I64LtU),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(1),ArgsEnum::U64(1),opcodes::I64LtU),ArgsEnum::Bool(true));
        //i64gts
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-1),ArgsEnum::I64(1),opcodes::I64GtS),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(-1),opcodes::I64GtS),ArgsEnum::Bool(true));
        //i64gtu
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(1),ArgsEnum::U64(2),opcodes::I64GtU),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(2),ArgsEnum::U64(1),opcodes::I64GtU),ArgsEnum::Bool(true));
        //i64les
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-1),ArgsEnum::I64(1),opcodes::I64LeS),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(-1),opcodes::I64LeS),ArgsEnum::Bool(false));
        //i64leu
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(1),ArgsEnum::U64(2),opcodes::I64LeU),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(2),ArgsEnum::U64(1),opcodes::I64LeU),ArgsEnum::Bool(false));
        //i64ges
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-1),ArgsEnum::I64(1),opcodes::I64GeS),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(1),ArgsEnum::I64(-1),opcodes::I64GeS),ArgsEnum::Bool(true));
        //i64geu
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(2),ArgsEnum::U64(1),opcodes::I64GeU),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(1),ArgsEnum::U64(2),opcodes::I64GeU),ArgsEnum::Bool(false));

        //f32eq
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(1.0),opcodes::F32Eq),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(2.0),opcodes::F32Eq),ArgsEnum::Bool(false));
        //f32ne
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(1.0),opcodes::F32Ne),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(2.0),opcodes::F32Ne),ArgsEnum::Bool(true));
        //f32lt
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(2.0),opcodes::F32Lt),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(2.0),ArgsEnum::F32(1.0),opcodes::F32Lt),ArgsEnum::Bool(false));
        //f32gt
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(2.0),opcodes::F32Gt),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(2.0),ArgsEnum::F32(1.0),opcodes::F32Gt),ArgsEnum::Bool(true));
        //f32le
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(2.0),opcodes::F32Le),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(2.0),ArgsEnum::F32(1.0),opcodes::F32Le),ArgsEnum::Bool(false));
        //f32ge
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(1.0),ArgsEnum::F32(2.0),opcodes::F32Ge),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::F32(2.0),ArgsEnum::F32(1.0),opcodes::F32Ge),ArgsEnum::Bool(true));

        //f64eq
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(1.0),opcodes::F64Eq),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(2.0),opcodes::F64Eq),ArgsEnum::Bool(false));
        //f64ne
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(1.0),opcodes::F64Ne),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(2.0),opcodes::F64Ne),ArgsEnum::Bool(true));
        //f64lt
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(2.0),opcodes::F64Lt),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(2.0),ArgsEnum::F64(1.0),opcodes::F64Lt),ArgsEnum::Bool(false));
        //f64gt
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(2.0),opcodes::F64Gt),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(2.0),ArgsEnum::F64(1.0),opcodes::F64Gt),ArgsEnum::Bool(true));
        //f64le
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(2.0),opcodes::F64Le),ArgsEnum::Bool(true));
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(2.0),ArgsEnum::F64(1.0),opcodes::F64Le),ArgsEnum::Bool(false));
        //f64ge
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(1.0),ArgsEnum::F64(2.0),opcodes::F64Ge),ArgsEnum::Bool(false));
        assert_eq!(none_args(&mut vm,ArgsEnum::F64(2.0),ArgsEnum::F64(1.0),opcodes::F64Ge),ArgsEnum::Bool(true));


        assert_eq!(none_args_2(&mut vm,ArgsEnum::U32(0xF0),opcodes::I32Clz),ArgsEnum::U32(24));
        assert_eq!(none_args_2(&mut vm,ArgsEnum::U32(0xF0),opcodes::I32Ctz),ArgsEnum::U32(4));
        assert_eq!(none_args_2(&mut vm,ArgsEnum::U32(0xF0F0),opcodes::I32PopCnt),ArgsEnum::U32(8));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(3),ArgsEnum::U32(2),opcodes::I32Add),ArgsEnum::U32(5));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(3),ArgsEnum::U32(2),opcodes::I32Sub),ArgsEnum::U32(1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(3),ArgsEnum::U32(2),opcodes::I32Mul),ArgsEnum::U32(6));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(8),ArgsEnum::U32(2),opcodes::I32DivU),ArgsEnum::U32(4));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-8),ArgsEnum::I32(4),opcodes::I32DivS),ArgsEnum::I32(-2));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-5),ArgsEnum::I32(2),opcodes::I32RemS),ArgsEnum::I32(-1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(-5_i32 as u32),ArgsEnum::U32(2),opcodes::I32RemU),ArgsEnum::U32(1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(0x0F0F),ArgsEnum::U32(0xF00F),opcodes::I32Or),ArgsEnum::U32(0xFF0F));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(0x0F0F),ArgsEnum::U32(0xF00F),opcodes::I32Xor),ArgsEnum::U32(0xFF00));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(-1_i32 as u32),ArgsEnum::U32(8),opcodes::I32Shl),ArgsEnum::U32(-256_i32 as u32));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(-1_i32 as u32),ArgsEnum::U32(200),opcodes::I32Shl),ArgsEnum::U32(-256_i32 as u32));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-1_i32),ArgsEnum::I32(8),opcodes::I32ShrS),ArgsEnum::I32(-1_i32));
        assert_eq!(none_args(&mut vm,ArgsEnum::I32(-1_i32),ArgsEnum::I32(200),opcodes::I32ShrS),ArgsEnum::I32(-1_i32));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(-1_i32 as u32),ArgsEnum::U32(8),opcodes::I32ShrU),ArgsEnum::U32(0xFF_FFFF));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(-1_i32 as u32),ArgsEnum::U32(200),opcodes::I32ShrU),ArgsEnum::U32(0xFF_FFFF));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(0x1234_5678),ArgsEnum::U32(8),opcodes::I32Rotl),ArgsEnum::U32(0x3456_7812));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(0x1234_5678),ArgsEnum::U32(200),opcodes::I32Rotl),ArgsEnum::U32(0x3456_7812));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(0x1234_5678),ArgsEnum::U32(8),opcodes::I32Rotr),ArgsEnum::U32(0x7812_3456));
        assert_eq!(none_args(&mut vm,ArgsEnum::U32(0x1234_5678),ArgsEnum::U32(200),opcodes::I32Rotr),ArgsEnum::U32(0x7812_3456));

        assert_eq!(none_args_2(&mut vm,ArgsEnum::U64(0xF0),opcodes::I64Clz),ArgsEnum::U64(56));
        assert_eq!(none_args_2(&mut vm,ArgsEnum::U64(0xF0),opcodes::I64Ctz),ArgsEnum::U64(4));
        assert_eq!(none_args_2(&mut vm,ArgsEnum::U64(0xF0F0),opcodes::I64PopCnt),ArgsEnum::U64(8));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(3),ArgsEnum::U64(2),opcodes::I64Add),ArgsEnum::U64(5));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(3),ArgsEnum::U64(2),opcodes::I64Sub),ArgsEnum::U64(1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(3),ArgsEnum::U64(2),opcodes::I64Mul),ArgsEnum::U64(6));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-8),ArgsEnum::I64(2),opcodes::I64DivS),ArgsEnum::I64(-4));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(-8_i64 as u64),ArgsEnum::U64(2),opcodes::I64DivU),ArgsEnum::U64(0x7FFF_FFFF_FFFF_FFFC));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-5),ArgsEnum::I64(2),opcodes::I64RemS),ArgsEnum::I64(-1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(-5_i64 as u64),ArgsEnum::U64(2),opcodes::I64RemU),ArgsEnum::U64(1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(0x0F0F),ArgsEnum::U64(0xF00F),opcodes::I64And),ArgsEnum::U64(0x000F));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(0x0F0F),ArgsEnum::U64(0xF00F),opcodes::I64Or),ArgsEnum::U64(0xFF0F));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(-1_i64 as u64),ArgsEnum::U64(8),opcodes::I64Shl),ArgsEnum::U64(-256_i64 as u64));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(-1_i64 as u64),ArgsEnum::U64(200),opcodes::I64Shl),ArgsEnum::U64(-256_i64 as u64));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-1),ArgsEnum::I64(8),opcodes::I64ShrS),ArgsEnum::I64(-1));
        assert_eq!(none_args(&mut vm,ArgsEnum::I64(-1),ArgsEnum::I64(200),opcodes::I64ShrS),ArgsEnum::I64(-1));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(-1_i64 as u64),ArgsEnum::U64(8),opcodes::I64ShrU),ArgsEnum::U64(0xFF_FFFF_FFFF_FFFF));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(-1_i64 as u64),ArgsEnum::U64(200),opcodes::I64ShrU),ArgsEnum::U64(0xFF_FFFF_FFFF_FFFF));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(0x1234_5678_1234_5678),ArgsEnum::U64(8),opcodes::I64Rotl),ArgsEnum::U64(0x3456_7812_3456_7812));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(0x1234_5678_1234_5678),ArgsEnum::U64(200),opcodes::I64Rotl),ArgsEnum::U64(0x3456_7812_3456_7812));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(0x1234_5678_1234_5678),ArgsEnum::U64(8),opcodes::I64Rotr),ArgsEnum::U64(0x7812_3456_7812_3456));
        assert_eq!(none_args(&mut vm,ArgsEnum::U64(0x1234_5678_1234_5678),ArgsEnum::U64(200),opcodes::I64Rotr),ArgsEnum::U64(0x7812_3456_7812_3456));

        assert_eq!(none_args_2(&mut vm,F32(-1.5),opcodes::F32Abs),F32(1.5));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::F32Neg),F32(-1.5));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::F32Ceil),F32(2.0));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::F32Floor),F32(1.0));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::F32Trunc),F32(1.0));
        assert_eq!(none_args_2(&mut vm,F32(0.5),opcodes::F32Nearest),F32(1.0));
        assert_eq!(none_args_2(&mut vm,F32(1.1),opcodes::F32Nearest),F32(1.0));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::F32Nearest),F32(2.0));
        assert_eq!(none_args_2(&mut vm,F32(1.9),opcodes::F32Nearest),F32(2.0));
        assert_eq!(none_args_2(&mut vm,F32(-0.5),opcodes::F32Nearest),F32(-1.0));
        assert_eq!(none_args_2(&mut vm,F32(4.0),opcodes::F32Sqrt),F32(2.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32Add),F32(5.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32Sub),F32(1.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32Mul),F32(6.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32Div),F32(1.5));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32Min),F32(2.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32Max),F32(3.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(2.0),opcodes::F32CopySign),F32(3.0));
        assert_eq!(none_args(&mut vm,F32(3.0),F32(-2.0),opcodes::F32CopySign),F32(-3.0));

        assert_eq!(none_args_2(&mut vm,F64(-1.5),opcodes::F64Abs),F64(1.5));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::F64Neg),F64(-1.5));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::F64Ceil),F64(2.0));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::F64Floor),F64(1.0));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::F64Trunc),F64(1.0));
        assert_eq!(none_args_2(&mut vm,F64(0.5),opcodes::F64Nearest),F64(1.0));
        assert_eq!(none_args_2(&mut vm,F64(1.1),opcodes::F64Nearest),F64(1.0));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::F64Nearest),F64(2.0));
        assert_eq!(none_args_2(&mut vm,F64(1.9),opcodes::F64Nearest),F64(2.0));
        assert_eq!(none_args_2(&mut vm,F64(-0.5),opcodes::F64Nearest),F64(-1.0));
        assert_eq!(none_args_2(&mut vm,F64(4.0),opcodes::F64Sqrt),F64(2.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64Add),F64(5.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64Sub),F64(1.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64Mul),F64(6.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64Div),F64(1.5));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64Min),F64(2.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64Max),F64(3.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(2.0),opcodes::F64CopySign),F64(3.0));
        assert_eq!(none_args(&mut vm,F64(3.0),F64(-2.0),opcodes::F64CopySign),F64(-3.0));

        assert_eq!(none_args_2(&mut vm,U64(0x7F7F_7F7F_7F7F_7F7F),opcodes::I32WrapI64),U32(0x7F7F_7F7F));
        assert_eq!(none_args_2(&mut vm,F32(-1.5),opcodes::I32TruncF32S),I32(-1));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::I32TruncF32U),U32(1));
        assert_eq!(none_args_2(&mut vm,F64(-1.5),opcodes::I32TruncF64S),I32(-1));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::I32TruncF64U),U32(1));
        assert_eq!(none_args_2(&mut vm,I32(-1),opcodes::I64ExtendI32S),I64(-1));
        assert_eq!(none_args_2(&mut vm,U32(-1_i32 as u32),opcodes::I64ExtendI32U),U64(0xFFFF_FFFF));
        assert_eq!(none_args_2(&mut vm,F32(-1.5),opcodes::I64TruncF32S),I64(-1));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::I64TruncF32U),U64(1));
        assert_eq!(none_args_2(&mut vm,F64(-1.5),opcodes::I64TruncF64S),I64(-1));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::I64TruncF64U),U64(1));
        assert_eq!(none_args_2(&mut vm,I32(-1),opcodes::F32ConvertI32S),F32(-1.0));
        assert_eq!(none_args_2(&mut vm,U32(-1_i32 as u32),opcodes::F32ConvertI32U),F32(4.2949673e+09));
        assert_eq!(none_args_2(&mut vm,I64(-1),opcodes::F32ConvertI64S),F32(-1.0));
        assert_eq!(none_args_2(&mut vm,U64(-1_i64 as u64),opcodes::F32ConvertI64U),F32(1.8446744e+19));
        assert_eq!(none_args_2(&mut vm,F64(1.5),opcodes::F32DemoteF64),F32(1.5));
        assert_eq!(none_args_2(&mut vm,I32(-1),opcodes::F64ConvertI32S),F64((-1.0)));
        assert_eq!(none_args_2(&mut vm,U32(-1_i32 as u32),opcodes::F64ConvertI32U),F64(4.294967295e+09));
        assert_eq!(none_args_2(&mut vm,I64(-1),opcodes::F64ConvertI64S),F64(-1.0));
        assert_eq!(none_args_2(&mut vm,U64(-1_i64 as u64),opcodes::F64ConvertI64U),F64(1.8446744073709552e+19));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::F64PromoteF32),F64(1.5));
        assert_eq!(none_args_2(&mut vm,F32(1.5),opcodes::I32ReinterpretF32),I32(0x3FC0_0000));
    }
}