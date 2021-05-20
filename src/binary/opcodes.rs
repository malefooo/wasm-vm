
use std::collections::HashMap;
use once_cell::sync::OnceCell;

pub static OPCODE_MAP:OnceCell<HashMap<u8,&str>> = OnceCell::new();

pub fn init(){
    let mut map:HashMap<u8,&str> = HashMap::new();
    map.insert(Unreachable,"Unreachable");
    map.insert(Nop,"nop");
    map.insert(Block,"block");
    map.insert(Loop,"loop");
    map.insert(If,"u");
    map.insert(Else_,"else");
    map.insert(End_,"end");
    map.insert(Br,"br");
    map.insert(BrIf,"br_if");
    map.insert(BrTable,"br_table");
    map.insert(Return, "return");
    map.insert(Call,"call");
    map.insert(CallIndirect,"call_indirect");
    map.insert(Drop,"drop");
    map.insert(Select,"select");
    map.insert(LocalGet,"local.get");
    map.insert(LocalSet,"local.set");
    map.insert(LocalTee,"local.tee");
    map.insert(GlobalGet,"global.get");
    map.insert(GlobalSet,"global.set");
    map.insert(I32Load,"i32.load");
    map.insert(I64Load,"i64.load");
    map.insert(F64Load,"f64.load");
    map.insert(I32Load8S,"i32.load8_s");
    map.insert(I32Load8U,"i32.load8_u");
    map.insert(I32Load16S,"i32.load16_s");
    map.insert(I32Load16U,"i32.load16_u");
    map.insert(I64Load8S,"i64.load8_s");
    map.insert(I64Load8U,"i64.load8_u");
    map.insert(I64Load16S,"i64.load16_s");
    map.insert(I64Load16U,"i64.load16_u");
    map.insert(I64Load32S,"i64.load32_s");
    map.insert(I64Load32U,"i64.load32_u");
    map.insert(I32Store,"i32.store");
    map.insert(I64Store,"i64.store");
    map.insert(F32Store,"f32.store");
    map.insert(F64Store,"f64.store");
    map.insert(I32Store8,"i32.store8");
    map.insert(I32Store16,"i32.store16");
    map.insert(I64Store8,"i64.store8");
    map.insert(I64Store16,"i64.store16");
    map.insert(I64Store32,"i64.store32");
    map.insert(MemorySize,"memory.size");
    map.insert(MemoryGrow,"memory,grow");
    map.insert(I32Const,"i32.const");
    map.insert(I64Const,"i64.const");
    map.insert(F32Const,"f32.const");
    map.insert(F64Const,"f64.const");
    map.insert(I32Eqz,"i32.eqz");
    map.insert(I32Eq,"i32.eq");
    map.insert(I32Ne,"i32.ne");
    map.insert(I32LtS,"i32.lts");
    map.insert(I32LtU,"i32.ltu");
    map.insert(I32GtS,"i32.gts");
    map.insert(I32GtU,"i32.gtu");
    map.insert(I32LeS,"i32.les");
    map.insert(I32LeU,"i32.leu");
    map.insert(I32GeS,"i32.ges");
    map.insert(I32GeU,"i32.geu");
    map.insert(I64Eqz,"i64.eqz");
    map.insert(I64Eq,"i64.eq");
    map.insert(I64Ne,"i64.ne");
    map.insert(I64LtS,"i64.lts");
    map.insert(I64LtU,"i64.ltu");
    map.insert(I64GtS,"i64.gts");
    map.insert(I64GtU,"i64.gtu");
    map.insert(I64LeS,"i64.les");
    map.insert(I64LeU,"i64.leu");
    map.insert(I64GeS,"i64.ges");
    map.insert(I64GeU,"i64.geu");
    map.insert(F32Eq,"f32.eq");
    map.insert(F32Ne,"f32.ne");
    map.insert(F32Lt,"f32.lt");
    map.insert(F32Gt,"f32.gt");
    map.insert(F32Le,"f32.le");
    map.insert(F32Ge,"f32.ge");
    map.insert(F64Eq,"f64.eq");
    map.insert(F64Ne,"f64.ne");
    map.insert(F64Lt,"f64.lt");
    map.insert(F64Gt,"f64.gt");
    map.insert(F64Le,"f64.le");
    map.insert(F64Ge,"f64.ge");
    map.insert(I32Clz,"i32.clz");
    map.insert(I32Ctz,"i32.ctz");
    map.insert(I32PopCnt,"i32.popcnt");
    map.insert(I32Add,"i32.add");
    map.insert(I32Sub,"i32.sub");
    map.insert(I32Mul,"i32.mul");
    map.insert(I32DivS,"i32.div_s");
    map.insert(I32DivU,"i32.div_u");
    map.insert(I32RemS,"i32.rem_s");
    map.insert(I32RemU,"i32.rem_u");
    map.insert(I32And,"i32.and");
    map.insert(I32Or,"i32.or");
    map.insert(I32Xor,"i32.xor");
    map.insert(I32Shl,"i32.shl");
    map.insert(I32ShrS,"i32.shr_s");
    map.insert(I32ShrU,"i32.shr_u");
    map.insert(I32Rotl,"i32.rotl");
    map.insert(I32Rotr,"i32.rotr");
    map.insert(I64Clz,"i64.clz");
    map.insert(I64Ctz,"i64.ctz");
    map.insert(I64PopCnt,"i64.popcnt");
    map.insert(I64Add,"i64.add");
    map.insert(I64Sub,"i64.sub");
    map.insert(I64Mul,"i64.mul");
    map.insert(I64DivS,"i64.div_s");
    map.insert(I64DivU,"i64.div_u");
    map.insert(I64RemS,"i64.rem_s");
    map.insert(I64RemU,"i64.rem_u");
    map.insert(I64And,"i64.and");
    map.insert(I64Or,"i64.or");
    map.insert(I64Xor,"i64.xor");
    map.insert(I64Shl,"i64.shl");
    map.insert(I64ShrS,"i64.shr_s");
    map.insert(I64ShrU,"i64.shr_u");
    map.insert(I64Rotl,"i64_rotl");
    map.insert(I64Rotr,"i64.rotr");
    map.insert(F32Abs,"f32.abs");
    map.insert(F32Neg,"f32.neg");
    map.insert(F32Ceil,"f32.ceil");
    map.insert(F32Floor,"f32.floor");
    map.insert(F32Trunc,"f32.trunc");
    map.insert(F32Nearest,"f32.nearest");
    map.insert(F32Sqrt,"f32.sqrt");
    map.insert(F32Add,"f32.add");
    map.insert(F32Sub,"f32.sub");
    map.insert(F32Mul,"f32.mul");
    map.insert(F32Div,"f32.div");
    map.insert(F32Min,"f32.min");
    map.insert(F32Max,"f32.max");
    map.insert(F32CopySign,"f32.copysign");
    map.insert(F64Abs,"f64.abs");
    map.insert(F64Neg,"f64.neg");
    map.insert(F64Ceil,"f64.ceil");
    map.insert(F64Floor,"f64.floor");
    map.insert(F64Trunc,"f64.trunc");
    map.insert(F64Nearest,"f64.nearest");
    map.insert(F64Sqrt,"f64.sqrt");
    map.insert(F64Add,"f64.add");
    map.insert(F64Sub,"f64.sub");
    map.insert(F64Mul,"f64.mul");
    map.insert(F64Div,"f64.div");
    map.insert(F64Min,"f64.min");
    map.insert(F64Max,"f64.max");
    map.insert(F64CopySign,"f64.copysign");
    map.insert(I32WrapI64,"i32.wrap_i64");
    map.insert(I32TruncF32S,"i32.trunc_f32_s");
    map.insert(I32TruncF32U,"i32_trunc_f32_u");
    map.insert(I32TruncF64S,"i32_trunc_f64_s");
    map.insert(I32TruncF64U,"i32_trunc_f64_u");
    map.insert(I64ExtendI32S,"i64.extend_i32_s");
    map.insert(I64ExtendI32U,"i64.extend_i32_u");
    map.insert(I64TruncF32S,"i64.trunc_f32_s");
    map.insert(I64TruncF32U,"i64.trunc_f32_u");
    map.insert(I64TruncF64S,"i64.trunc_f64");
    map.insert(F32ConvertI32S,"f32.convert_i32_s");
    map.insert(F32ConvertI32U,"f32.convert_i32_u");
    map.insert(F32ConvertI64S,"f32.convert_i64_s");
    map.insert(F32ConvertI64U,"f32.convert_i64_u");
    map.insert(F32DemoteF64,"f32.demote_f64");
    map.insert(F64ConvertI32S,"f64.convert_i32_s");
    map.insert(F64ConvertI32U,"f64.convert_i32_u");
    map.insert(F64ConvertI64S,"f64.convert_i64_s");
    map.insert(F64ConvertI64U,"f64.convert_i64_u");
    map.insert(F64PromoteF32,"f64.promote_f32");
    map.insert(I32ReinterpretF32,"i32.reinterpret_f32");
    map.insert(I64ReinterpretF64,"i64.reinterpret_i64");
    map.insert(F32ReinterpretI32,"f32.reinterpret_i32");
    map.insert(F64ReinterpretI64,"f64.reinterpret_i64");
    map.insert(I32Extend8S,"i32.extend8_s");
    map.insert(I32Extend16S,"i32.extend16_s");
    map.insert(I64Extend8S,"i64.extend8_s");
    map.insert(I64Extend16S,"i64.extend16_s");
    map.insert(I64Extend32S,"i64.extend32_s");
    map.insert(TruncSat,"trunc_sat");

    OPCODE_MAP.set(map);

}

pub const Unreachable       :u8= 0x00; // unreachable
pub const Nop               :u8= 0x01; // nop
pub const Block             :u8= 0x02; // block rt in* end
pub const Loop              :u8= 0x03; // loop rt in* end
pub const If                :u8= 0x04; // if rt in* else in* end
pub const Else_             :u8= 0x05; // else
pub const End_              :u8= 0x0B; // end
pub const Br                :u8= 0x0C; // br l
pub const BrIf              :u8= 0x0D; // br_if l
pub const BrTable           :u8= 0x0E; // br_table l* lN
pub const Return            :u8= 0x0F; // return
pub const Call              :u8= 0x10; // call x
pub const CallIndirect      :u8= 0x11; // call_indirect x
pub const Drop              :u8= 0x1A; // drop
pub const Select            :u8= 0x1B; // select
pub const LocalGet          :u8= 0x20; // local.get x
pub const LocalSet          :u8= 0x21; // local.set x
pub const LocalTee          :u8= 0x22; // local.tee x
pub const GlobalGet         :u8= 0x23; // global.get x
pub const GlobalSet         :u8= 0x24; // global.set x
pub const I32Load           :u8= 0x28; // i32.load m
pub const I64Load           :u8= 0x29; // i64.load m
pub const F32Load           :u8= 0x2A; // f32.load m
pub const F64Load           :u8= 0x2B; // f64.load m
pub const I32Load8S         :u8= 0x2C; // i32.load8_s m
pub const I32Load8U         :u8= 0x2D; // i32.load8_u m
pub const I32Load16S        :u8= 0x2E; // i32.load16_s m
pub const I32Load16U        :u8= 0x2F; // i32.load16_u m
pub const I64Load8S         :u8= 0x30; // i64.load8_s m
pub const I64Load8U         :u8= 0x31; // i64.load8_u m
pub const I64Load16S        :u8= 0x32; // i64.load16_s m
pub const I64Load16U        :u8= 0x33; // i64.load16_u m
pub const I64Load32S        :u8= 0x34; // i64.load32_s m
pub const I64Load32U        :u8= 0x35; // i64.load32_u m
pub const I32Store          :u8= 0x36; // i32.store m
pub const I64Store          :u8= 0x37; // i64.store m
pub const F32Store          :u8= 0x38; // f32.store m
pub const F64Store          :u8= 0x39; // f64.store m
pub const I32Store8         :u8= 0x3A; // i32.store8 m
pub const I32Store16        :u8= 0x3B; // i32.store16 m
pub const I64Store8         :u8= 0x3C; // i64.store8 m
pub const I64Store16        :u8= 0x3D; // i64.store16 m
pub const I64Store32        :u8= 0x3E; // i64.store32 m
pub const MemorySize        :u8= 0x3F; // memory.size
pub const MemoryGrow        :u8= 0x40; // memory.grow
pub const I32Const          :u8= 0x41; // i32.const n
pub const I64Const          :u8= 0x42; // i64.const n
pub const F32Const          :u8= 0x43; // f32.const z
pub const F64Const          :u8= 0x44; // f64.const z
pub const I32Eqz            :u8= 0x45; // i32.eqz
pub const I32Eq             :u8= 0x46; // i32.eq
pub const I32Ne             :u8= 0x47; // i32.ne
pub const I32LtS            :u8= 0x48; // i32.lt_s
pub const I32LtU            :u8= 0x49; // i32.lt_u
pub const I32GtS            :u8= 0x4A; // i32.gt_s
pub const I32GtU            :u8= 0x4B; // i32.gt_u
pub const I32LeS            :u8= 0x4C; // i32.le_s
pub const I32LeU            :u8= 0x4D; // i32.le_u
pub const I32GeS            :u8= 0x4E; // i32.ge_s
pub const I32GeU            :u8= 0x4F; // i32.ge_u
pub const I64Eqz            :u8= 0x50; // i64.eqz
pub const I64Eq             :u8= 0x51; // i64.eq
pub const I64Ne             :u8= 0x52; // i64.ne
pub const I64LtS            :u8= 0x53; // i64.lt_s
pub const I64LtU            :u8= 0x54; // i64.lt_u
pub const I64GtS            :u8= 0x55; // i64.gt_s
pub const I64GtU            :u8= 0x56; // i64.gt_u
pub const I64LeS            :u8= 0x57; // i64.le_s
pub const I64LeU            :u8= 0x58; // i64.le_u
pub const I64GeS            :u8= 0x59; // i64.ge_s
pub const I64GeU            :u8= 0x5A; // i64.ge_u
pub const F32Eq             :u8= 0x5B; // f32.eq
pub const F32Ne             :u8= 0x5C; // f32.ne
pub const F32Lt             :u8= 0x5D; // f32.lt
pub const F32Gt             :u8= 0x5E; // f32.gt
pub const F32Le             :u8= 0x5F; // f32.le
pub const F32Ge             :u8= 0x60; // f32.ge
pub const F64Eq             :u8= 0x61; // f64.eq
pub const F64Ne             :u8= 0x62; // f64.ne
pub const F64Lt             :u8= 0x63; // f64.lt
pub const F64Gt             :u8= 0x64; // f64.gt
pub const F64Le             :u8= 0x65; // f64.le
pub const F64Ge             :u8= 0x66; // f64.ge
pub const I32Clz            :u8= 0x67; // i32.clz
pub const I32Ctz            :u8= 0x68; // i32.ctz
pub const I32PopCnt         :u8= 0x69; // i32.popcnt
pub const I32Add            :u8= 0x6A; // i32.add
pub const I32Sub            :u8= 0x6B; // i32.sub
pub const I32Mul            :u8= 0x6C; // i32.mul
pub const I32DivS           :u8= 0x6D; // i32.div_s
pub const I32DivU           :u8= 0x6E; // i32.div_u
pub const I32RemS           :u8= 0x6F; // i32.rem_s
pub const I32RemU           :u8= 0x70; // i32.rem_u
pub const I32And            :u8= 0x71; // i32.and
pub const I32Or             :u8= 0x72; // i32.or
pub const I32Xor            :u8= 0x73; // i32.xor
pub const I32Shl            :u8= 0x74; // i32.shl
pub const I32ShrS           :u8= 0x75; // i32.shr_s
pub const I32ShrU           :u8= 0x76; // i32.shr_u
pub const I32Rotl           :u8= 0x77; // i32.rotl
pub const I32Rotr           :u8= 0x78; // i32.rotr
pub const I64Clz            :u8= 0x79; // i64.clz
pub const I64Ctz            :u8= 0x7A; // i64.ctz
pub const I64PopCnt         :u8= 0x7B; // i64.popcnt
pub const I64Add            :u8= 0x7C; // i64.add
pub const I64Sub            :u8= 0x7D; // i64.sub
pub const I64Mul            :u8= 0x7E; // i64.mul
pub const I64DivS           :u8= 0x7F; // i64.div_s
pub const I64DivU           :u8= 0x80; // i64.div_u
pub const I64RemS           :u8= 0x81; // i64.rem_s
pub const I64RemU           :u8= 0x82; // i64.rem_u
pub const I64And            :u8= 0x83; // i64.and
pub const I64Or             :u8= 0x84; // i64.or
pub const I64Xor            :u8= 0x85; // i64.xor
pub const I64Shl            :u8= 0x86; // i64.shl
pub const I64ShrS           :u8= 0x87; // i64.shr_s
pub const I64ShrU           :u8= 0x88; // i64.shr_u
pub const I64Rotl           :u8= 0x89; // i64.rotl
pub const I64Rotr           :u8= 0x8A; // i64.rotr
pub const F32Abs            :u8= 0x8B; // f32.abs
pub const F32Neg            :u8= 0x8C; // f32.neg
pub const F32Ceil           :u8= 0x8D; // f32.ceil
pub const F32Floor          :u8= 0x8E; // f32.floor
pub const F32Trunc          :u8= 0x8F; // f32.trunc
pub const F32Nearest        :u8= 0x90; // f32.nearest
pub const F32Sqrt           :u8= 0x91; // f32.sqrt
pub const F32Add            :u8= 0x92; // f32.add
pub const F32Sub            :u8= 0x93; // f32.sub
pub const F32Mul            :u8= 0x94; // f32.mul
pub const F32Div            :u8= 0x95; // f32.div
pub const F32Min            :u8= 0x96; // f32.min
pub const F32Max            :u8= 0x97; // f32.max
pub const F32CopySign       :u8= 0x98; // f32.copysign
pub const F64Abs            :u8= 0x99; // f64.abs
pub const F64Neg            :u8= 0x9A; // f64.neg
pub const F64Ceil          :u8 = 0x9B; // f64.ceil
pub const F64Floor        :u8  = 0x9C; // f64.floor
pub const F64Trunc     :u8     = 0x9D; // f64.trunc
pub const F64Nearest:u8        = 0x9E; // f64.nearest
pub const F64Sqrt:u8           = 0x9F; // f64.sqrt
pub const F64Add            :u8= 0xA0; // f64.add
pub const F64Sub            :u8= 0xA1; // f64.sub
pub const F64Mul            :u8= 0xA2; // f64.mul
pub const F64Div            :u8= 0xA3; // f64.div
pub const F64Min            :u8= 0xA4; // f64.min
pub const F64Max            :u8= 0xA5; // f64.max
pub const F64CopySign       :u8= 0xA6; // f64.copysign
pub const I32WrapI64        :u8= 0xA7; // i32.wrap_i64
pub const I32TruncF32S      :u8= 0xA8; // i32.trunc_f32_s
pub const I32TruncF32U      :u8= 0xA9; // i32.trunc_f32_u
pub const I32TruncF64S      :u8= 0xAA; // i32.trunc_f64_s
pub const I32TruncF64U      :u8= 0xAB; // i32.trunc_f64_u
pub const I64ExtendI32S     :u8= 0xAC; // i64.extend_i32_s
pub const I64ExtendI32U     :u8= 0xAD; // i64.extend_i32_u
pub const I64TruncF32S      :u8= 0xAE; // i64.trunc_f32_s
pub const I64TruncF32U      :u8= 0xAF; // i64.trunc_f32_u
pub const I64TruncF64S      :u8= 0xB0; // i64.trunc_f64_s
pub const I64TruncF64U      :u8= 0xB1; // i64.trunc_f64_u
pub const F32ConvertI32S    :u8= 0xB2; // f32.convert_i32_s
pub const F32ConvertI32U    :u8= 0xB3; // f32.convert_i32_u
pub const F32ConvertI64S    :u8= 0xB4; // f32.convert_i64_s
pub const F32ConvertI64U   :u8 = 0xB5; // f32.convert_i64_u
pub const F32DemoteF64     :u8 = 0xB6; // f32.demote_f64
pub const F64ConvertI32S  :u8  = 0xB7; // f64.convert_i32_s
pub const F64ConvertI32U :u8   = 0xB8; // f64.convert_i32_u
pub const F64ConvertI64S:u8    = 0xB9; // f64.convert_i64_s
pub const F64ConvertI64U:u8    = 0xBA; // f64.convert_i64_u
pub const F64PromoteF32:u8     = 0xBB; // f64.promote_f32
pub const I32ReinterpretF32:u8 = 0xBC; // i32.reinterpret_f32
pub const I64ReinterpretF64:u8 = 0xBD; // i64.reinterpret_f64
pub const F32ReinterpretI32:u8 = 0xBE; // f32.reinterpret_i32
pub const F64ReinterpretI64:u8 = 0xBF; // f64.reinterpret_i64
pub const I32Extend8S:u8       = 0xC0; // i32.extend8_s
pub const I32Extend16S:u8      = 0xC1; // i32.extend16_s
pub const I64Extend8S:u8       = 0xC2; // i64.extend8_s
pub const I64Extend16S:u8      = 0xC3; // i64.extend16_s
pub const I64Extend32S:u8      = 0xC4; // i64.extend32_s
pub const TruncSat:u8          = 0xFC; // <i32|64>.trunc_sat_<f32|64>_<s|u>

