use std::string::FromUtf8Error;
use byteorder::{ByteOrder, LittleEndian};
use crate::binary::{leb128, module, opcodes, instruction};
use crate::binary::module::{CustomSecs, Module, FuncType, TableType, Limits};
use std::fs::{OpenOptions, read};
use std::io::Read;
use anyhow::Context;
use crate::binary::instruction::Instruction;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;


pub struct WasmReader{
    data:Vec<u8>,
}

pub fn decode_file(path:String) -> anyhow::Result<module::Module>{
    let v = std::fs::read(path)?;
    // println!("data:{:?}",v);
    decode(v)
}

pub fn decode(data:Vec<u8>) -> anyhow::Result<module::Module>{
    let mut reader = WasmReader{ data };
    reader.read_module().context("read module none")
}

/// 读取基础数据结构
impl WasmReader {

    /// 读取定长首字节,
    /// 读取玩之后除去第一位
    pub fn read_byte(&mut self) -> Option<u8>{

        let mut val:Option<u8> = Option::default();
        match self.data.get(0) {
            None => {val=None}
            Some(b) => {
                val = Some(*b)
            }
        }

        let v:Vec<u8> = self.data.drain(1..).collect();
        self.data = v;

        val
    }

    /// 读取定长u32
    /// 读完之后去除前4个
    pub fn read_u32(&mut self) -> Option<u32>{
        let val = Some(byteorder::LittleEndian::read_u32(self.data.as_slice()));
        self.data = self.data.drain(4..).collect();
        val
    }

    /// 读取定长f32
    /// 读完之后去除前4个
    pub fn read_f32(&mut self) -> Option<f32>{

        let val = Some(byteorder::LittleEndian::read_f32(self.data.as_slice()));
        self.data = self.data.drain(4..).collect();
        val
    }

    /// 读取定长f64
    /// 读完之后去除钱8个
    pub fn read_f64(&mut self) -> Option<f64>{

        let val = Some(byteorder::LittleEndian::read_f64(self.data.as_slice()));
        self.data = self.data.drain(8..).collect();
        val
    }

    /// 读取变长u32
    /// 根据leb128中返回的size
    pub fn read_var_u32(&mut self) -> Option<u32>{
        let mut val:Option<u32> = Option::default();
        match leb128::decode_var_uint(self.data.as_slice(),32){
            Ok((num,i)) => {
                val = Some(num as u32);
                let v:Vec<u8> = self.data.drain(i..).collect();
                self.data = v;
            }
            Err(e) => {
                println!("{:?}",e);
                val = None
            }
        }
        val
    }

    /// 读取变长i32
    /// 根据leb128返回的size截取vec
    pub fn read_var_s32(&mut self) -> Option<i32>{
        let mut val:Option<i32> = Option::default();
        match leb128::decode_var_int(self.data.as_slice(),32){
            Ok((num,i)) => {
                val = Some(num as i32);
                let v:Vec<u8> = self.data.drain(i..).collect();
                self.data = v;
            }
            Err(e) => {
                println!("{:?}",e);
                val = None
            }
        }
        val
    }

    /// 读取变长f64
    pub fn read_var_s64(&mut self) -> Option<i64>{
        let mut val:Option<i64> = Option::default();
        match leb128::decode_var_int(self.data.as_slice(),64){
            Ok((num,i)) => {
                val = Some(num);
                let v:Vec<u8> = self.data.drain(i..).collect();
                self.data = v;
            }
            Err(e) => {
                println!("{:?}",e);
                val = None
            }
        }
        val
    }

    /// 读取变长u32
    /// 返回0..n的vec
    /// 截取vec n..
    pub fn read_bytes(&mut self) -> Option<Vec<u8>>{
        let mut val:Option<Vec<u8>> = Option::default();
        match self.read_var_u32(){
            None => {val = None}
            Some(n) => {
                let v:Vec<u8> = self.data.drain(0..n as usize).collect();
                // let v2:Vec<u8> = self.data.drain(n as usize..).collect();
                val = Some(v);
                self.data = self.data.clone()
            }
        }

        val
    }

    /// 读取变长u32然后转为string
    pub fn read_name(&mut self) -> Option<String>{
        return match self.read_bytes(){
            None => {None}
            Some(v) => {
                String::from_utf8(v).ok()
            }
        }
    }

    /// 查询vec长度
    pub fn remaining(&self) -> usize{
        self.data.len()
    }

}


/// 读取段
impl WasmReader {

    /// 根据第一个单字节来判断导入的是什么类型的数据,读取导读描述
    pub fn read_import_desc(&mut self) -> Option<module::ImportDesc>{
        let mut desc = module::ImportDesc{
            tag: self.read_byte(),
            fun_type: None,
            table: None,
            mem: None,
            global: None
        };

        match desc.tag {
            None => {
                panic!("read import desc none")
            }
            Some(b) => {
                match b {
                    module::IMPORT_TAG_FUNC => {
                        desc.fun_type = self.read_var_u32();
                    },
                    module::IMPORT_TAG_TABLE => {
                        desc.table = self.read_table_type();
                    },
                    module::IMPORT_TAG_MEM => {
                        desc.mem = self.read_limits();
                    },
                    module::IMPORT_TAG_GLOBAL => {
                        desc.global = self.read_global_type();
                    },
                    _ => {
                        panic!("invalid import desc tag:{:?}",b);
                    }
                }
            }
        }

        Some(desc)
    }


    /// 读取代码段
    pub fn read_code(&mut self) -> Option<module::Code>{
        let mut code = module::Code{ locals: None, expr: None };
        match self.read_bytes() {
            None => {
                return None
            }
            Some(v) => {
                let mut code_reader = WasmReader{ data: v };
                code.locals = code_reader.read_locals_vec();
            }
        }

        Some(code)
    }

    pub fn read_locals_vec(&mut self) -> Option<Vec<module::Locals>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::Locals> = Vec::new();
                for _ in 0..n {
                    let locals = module::Locals{
                        n: self.read_var_u32(),
                        ty: self.read_val_type(),
                    };
                    v.push(locals);
                }
                Some(v)
            }
        }
    }



    pub fn read_module(&mut self) -> Option<module::Module>{
       return match self.read_sections(){
            Ok(m) => {
                Some(m)
            }
            Err(str) => {
                println!("{:?}",str);
                None
            }
        }
    }

    pub fn read_sections(&mut self) -> anyhow::Result<module::Module,&'static str>{
        let mut m = module::Module{
            magic: self.read_u32(),//读4个
            version: self.read_u32(),//读4个
            custom_secs: None,
            type_sec: None,
            import_sec: None,
            func_sec: None,
            table_sec: None,
            mem_sec: None,
            global_sec: None,
            export_sec: None,
            start_sec: None,
            elem_sec: None,
            code_sec: None,
            data_sec: None
        };
        let mut prev_sec_id = 0u8;

        while self.remaining() > 0 {
            match self.read_byte(){
                None => {
                    return anyhow::Result::Err("read byte is none")
                }
                Some(b) => {
                    if b == module::SEC_CUSTOM_ID {
                        if m.custom_secs.as_ref().is_none() {
                            let mut v:Vec<module::CustomSecs> = Vec::new();
                            match self.read_custom_sec(){
                                None => {
                                    return anyhow::Result::Err("read custom sec none")
                                }
                                Some(c) => {
                                    v.push(c);
                                }
                            }
                            m.custom_secs = Some(v);
                        } else {
                            let mut v = m.custom_secs.unwrap();
                            match self.read_custom_sec(){
                                None => {
                                    return anyhow::Result::Err("read custom sec none")
                                }
                                Some(c) => {
                                    v.push(c);
                                    m.custom_secs = Some(v);
                                }
                            }

                        }
                        continue
                    }

                    if b > module::SEC_DATA_ID {
                        return anyhow::Result::Err("the id large");
                    }

                    if b <= prev_sec_id {
                        return anyhow::Result::Err("less than prev sec id");
                    }

                    prev_sec_id = b;

                    match self.read_var_u32(){
                        None => {
                            return anyhow::Result::Err("read var u32 none");
                        }
                        Some(n) => {
                            let l = self.remaining();
                            self.read_non_custom_sec(b,&mut m);
                            if self.remaining() + n as usize != l {
                                panic!("section size mismatch");
                                // return anyhow::Result::Err("section size mismatch");
                            }
                        }
                    }

                }
            }
        }
        Ok(m)
    }


    pub fn read_non_custom_sec(&mut self, sec_id:u8, m: &mut module::Module) {
        match sec_id {
            module::SEC_TYPE_ID => m.type_sec = self.read_type_sec(),
            module::SEC_IMPORT_ID => m.import_sec = self.read_import_sec(),
            module::SEC_FUNC_ID => m.func_sec = self.read_indices(),
            module::SEC_TABLE_ID => m.table_sec = self.read_table_sec(),
            module::SEC_MEM_ID => m.mem_sec = self.read_mem_sec(),
            module::SEC_GLOBAL_ID => m.global_sec = self.read_global_sec(),
            module::SEC_EXPORT_ID => m.export_sec = self.read_export_sec(),
            module::SEC_START_ID => m.start_sec = self.read_start_sec(),
            module::SEC_ELEM_ID => m.elem_sec = self.read_elem_sec(),
            module::SEC_CODE_ID => m.code_sec = self.read_code_sec(),
            module::SEC_DATA_ID => m.data_sec = self.read_data_sec(),
            _ => {}
        }
    }

    pub fn read_custom_sec(&mut self) -> Option<module::CustomSecs>{
        match self.read_bytes() {
            None => {
                None
            }
            Some(v) => {
                let mut reader = WasmReader{ data: v };
                Some(module::CustomSecs{ name: reader.read_name(), bytes: reader.data })
            }
        }
    }

    pub fn read_type_sec(&mut self) -> Option<Vec<module::FuncType>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::FuncType> = Vec::new();
                for _ in 0..n {
                    match self.read_func_type() {
                        None => {
                            panic!("read func type none")
                        }
                        Some(ft) => {
                            v.push(ft);
                        }
                    }
                };
                println!("type-sec{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_func_type(&mut self) -> Option<module::FuncType>{
        let ft = module::FuncType{
            tag: self.read_byte(),
            param_types: self.read_val_types(),
            result_types: self.read_val_types(),
        };

        match ft.tag {
            None => {
                panic!("read func type none")
            }
            Some(n) => {
                if n != module::FT_TAG {
                    panic!("invalid elem type")
                }

                Some(ft)
            }
        }
        
    }

    pub fn read_import_sec(&mut self) -> Option<Vec<module::Import>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::Import> = Vec::new();
                for _ in 0..n {
                    v.push(self.read_import().unwrap());
                };
                println!("import-sec:{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_import(&mut self) -> Option<module::Import>{
        Some(module::Import{
            module: self.read_name(),
            name: self.read_name(),
            import_desc: self.read_import_desc(),
        })
    }

    pub fn read_indices(&mut self) -> Option<Vec<module::TypeIdx>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::TypeIdx> = Vec::new();
                for _ in 0..n {
                    match self.read_var_u32() {
                        None => {
                            panic!("read indices read var u32 none");
                        }
                        Some(num) => {
                            v.push(num);
                        }
                    }
                };
                println!("func-sec:{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_table_sec(&mut self) -> Option<Vec<module::TableType>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::TableType> = Vec::new();
                for _ in 0..n {
                    match self.read_table_type() {
                        None => {
                            panic!("read table sec read table type none")
                        }
                        Some(tt) => {
                            v.push(tt);
                        }
                    }
                };
                println!("table-sec:{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_table_type(&mut self) -> Option<module::TableType>{
        let tt = module::TableType{
            elem_type: self.read_byte(),
            limits: self.read_limits(),
        };

        match tt.elem_type {
            None => {None}
            Some(n) => {
                if n != module::FUNC_REF {
                    panic!("invalid element type:{:?}",n);
                }

                Some(tt)
            }
        }
    }

    pub fn read_limits(&mut self) -> Option<module::Limits>{

        let mut limits = module::Limits{
            tag: self.read_byte(),
            min: self.read_var_u32(),
            max: None
        };

        match limits.tag {
            None => {
                panic!("read limits tag is none");
            }
            Some(n) => {

                if n == 1 {
                    limits.max = self.read_var_u32();
                }
                Some(limits)
            }
        }
    }

    pub fn read_mem_sec(&mut self) -> Option<Vec<module::MemType>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::MemType> = Vec::new();
                for _ in 0..n {
                    match self.read_limits() {
                        None => {
                            panic!("read limits none")
                        }
                        Some(limits) => {
                            v.push(limits);
                        }
                    }
                };
                println!("mem-sec:{:?}",v);
                Some(v)
            }
        }
    }


    pub fn read_global_sec(&mut self) -> Option<Vec<module::GlobalSec>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::GlobalSec> = Vec::new();
                for _ in 0..n {
                    let g = module::GlobalSec{ ty: self.read_global_type(), init: self.read_expr() };
                    v.push(g);
                };
                println!("global-sec:{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_global_type(&mut self) -> Option<module::GlobalType>{

        let mut gt = module::GlobalType{
            val_type: self.read_val_type(),
            m: self.read_byte()
        };

        match gt.m {
            None => {
                panic!("read global type is none")
            }
            Some(n) => {
                match n {
                    module::MUT_CONST => {},
                    module::MUT_VAR => {},
                    _ => {
                        panic!("malformed mutability:{:?}",n)
                    },
                }

                gt.m = Some(n);

                Some(gt)
            }
        }


    }

    pub fn read_val_types(&mut self) -> Option<Vec<u8>>{
        match self.read_var_u32() {
            None => {None}
            Some(n) => {
                let mut v:Vec<u8> = Vec::new();
                for _ in 0..n {
                    match self.read_val_type() {
                        None => {
                            panic!("read val none")
                        }
                        Some(num) => {
                            v.push(num);
                        }
                    }
                }
                Some(v)
            }
        }
    }

    /// 这里进行一个处理,如果是0x41起,就连续取出,直到结尾是0x0b为止
    pub fn read_val_type(&mut self) -> Option<u8>{
        match self.read_byte() {
            None => {panic!("read val type read byte is none")}
            Some(mut n) => {
                match n {
                    module::VAL_TYPE_I32 => {},
                    module::VAL_TYPE_F32 => {},
                    module::VAL_TYPE_F64 => {},
                    module::VAL_TYPE_I64 => {},
                    _ => {
                        panic!("malformed value type:{:?}",n);
                    }
                }

                Some(n)
            }
        }
    }

    pub fn read_export_sec(&mut self) -> Option<Vec<module::Export>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::Export> = Vec::new();
                for _ in 0..n {
                    let mut export = module::Export{
                        name: self.read_name(),
                        desc: None
                    };

                    let desc = module::ExportDesc{
                        tag: self.read_byte(),
                        idx: self.read_var_u32(),
                    };

                    match desc.tag {
                        None => {
                            panic!("tag none")
                        }
                        Some(num) => {
                            match num {
                                module::EXPORT_TAG_FUNC|
                                module::EXPORT_TAG_TABLE|
                                module::EXPORT_TAG_MEM|
                                module::EXPORT_TAG_GLOBAL => {},
                                _ => {
                                    panic!("tag unknown")
                                }
                            }
                        }
                    }
                    export.desc = Some(desc);
                    v.push(export);
                };
                println!("export-sec:{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_start_sec(&mut self) -> Option<module::FuncIdx>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                println!("start-sec:{:?}",n);
                Some(n)
            }
        }
    }

    pub fn read_elem_sec(&mut self) -> Option<Vec<module::Elem>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::Elem> = Vec::new();
                for _ in 0..n {
                    let elem = module::Elem{
                        table: self.read_var_u32(),
                        offset: self.read_expr(),
                        init: self.read_indices(),
                    };

                    v.push(elem);
                };
                println!("elem-sec:{:?}",v);
                Some(v)
            }
        }
    }

    pub fn read_code_sec(&mut self) -> Option<Vec<module::Code>>{
       match self.read_var_u32() {
           None => {
               None
           }
           Some(n) => {
               let mut vc:Vec<module::Code> = Vec::new();
               for _ in 0..n {
                   match self.read_bytes() {
                       None => {
                           return None
                       }
                       Some(v) => {
                           let mut reader = WasmReader{ data: v };
                           let code = module::Code{ locals: reader.read_locals_vec(), expr: reader.read_expr() };
                           if code.get_local_count().unwrap() >= u32::MAX {
                               panic!("too many locals");
                           }
                           vc.push(code);
                       }
                   }
               }
               println!("code-sec:{:?}",vc);
               Some(vc)
           }
       }
    }


    pub fn read_data_sec(&mut self) -> Option<Vec<module::Data>>{
        match self.read_var_u32() {
            None => {
                None
            }
            Some(n) => {
                let mut v:Vec<module::Data> = Vec::new();
                for _ in 0..n {
                    let data = module::Data{
                        mem: self.read_var_u32(),
                        offset: self.read_expr(),
                        init: self.read_bytes(),
                    };

                    v.push(data);
                }
                println!("data-sec:{:?}",v);
                Some(v)
            }
        }
    }


}

impl WasmReader {
    pub fn read_expr(&mut self) -> Option<instruction::Expr>{
        match self.read_instructions(){
            None => {
                panic!("read expr none")
            }
            Some(p) => {
                if p.1 != opcodes::End_ {
                    panic!("invalid expr end:{:?}",p.1)
                }
                Some(p.0)
            }
        }
    }

    pub fn read_instructions(&mut self) -> Option<(instruction::Expr,u8)>{
        let mut v:Vec<instruction::Instruction> = Vec::new();
        let mut end = 0u8;
        loop {
            match self.read_instruction(){
                None => {panic!("read instr none")}
                Some(i) => {

                    if i.opcode.as_ref().is_none() {
                        panic!("opcode is none")
                    }
                    let code = i.opcode.unwrap();
                    if code == opcodes::Else_ || code == opcodes::End_ {
                        end = code;
                        break
                    }
                    v.push(i)
                }
            }
        }
        Some((v,end))
    }

    pub fn read_instruction(&mut self) -> Option<instruction::Instruction>{

        let map = opcodes::OPCODE_MAP.get().unwrap();
        match self.read_byte() {
            None => {panic!("read instrs none")}
            Some(n) => {
                match map.get(&n) {
                    None => {panic!("map get none:{:?}",n)}
                    Some(str) => {
                        if str.eq(&"") {
                            panic!("get string is empty")
                        } else {
                            Some(instruction::Instruction{
                                opcode: Some(n),
                                args: self.read_args(n)
                            })
                        }
                    }
                }
            }
        }
    }

    pub fn read_args(&mut self,opcode:u8) -> Option<instruction::ArgsEnum>{
        // println!("123");
        match opcode {
            opcodes::Block|opcodes::Loop => {
                self.read_block_args()
            },
            opcodes::If => {
                self.read_if_args()
            },
            opcodes::Br|opcodes::BrIf  => {
                match self.read_var_u32() {
                    None => {panic!("read u32 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::U32(n))
                    }
                }
            },
            opcodes::BrTable => {
                self.read_br_table_args()
            },
            opcodes::Call => {
                match self.read_var_u32() {
                    None => {panic!("read u32 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::U32(n))
                    }
                }
            },
            opcodes::CallIndirect => {
                self.read_call_indirect_args().and_then(|n|Some(instruction::ArgsEnum::U32(n)))
            },
            opcodes::LocalGet|opcodes::LocalSet|opcodes::LocalTee => {
                match self.read_var_u32() {
                    None => {panic!("read u32 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::U32(n))
                    }
                }
            },
            opcodes::GlobalSet|opcodes::GlobalGet => {
                match self.read_var_u32() {
                    None => {panic!("read u32 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::U32(n))
                    }
                }
            },
            opcodes::MemoryGrow|opcodes::MemorySize => {
                match self.read_zero() {
                    None => {panic!("read zero none")}
                    Some(n) => {Some(instruction::ArgsEnum::U8(n))}
                }
            },
            opcodes::I32Const => {
                match self.read_var_s32() {
                    None => {panic!("read s32 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::I32(n))
                    }
                }
            },
            opcodes::I64Const => {
                match self.read_var_s64() {
                    None => {panic!("read s64 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::I64(n))
                    }
                }
            },
            opcodes::F32Const => {
                match self.read_f32() {
                    None => {panic!("read f32 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::F32(n))
                    }
                }
            },
            opcodes::F64Const => {
                match self.read_f64() {
                    None => {panic!("read f64 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::F64(n))
                    }
                }
            },
            opcodes::TruncSat => {
                match self.read_byte() {
                    None => {panic!("read u8 none")}
                    Some(n) => {
                        Some(instruction::ArgsEnum::U8(n))
                    }
                }
            },
            _=>{
                if opcode >= opcodes::I32Load && opcode <= opcodes::I64Store32 {
                    self.read_mem_arg().and_then(|o|Some(instruction::ArgsEnum::MemArg(o)))
                } else {
                    None
                }

            }
        }
    }

    pub fn read_block_args(&mut self) -> Option<instruction::ArgsEnum>{

        match self.read_block_type() {
            None => {panic!("read block type none")}
            Some(i) => {
                match self.read_instructions() {
                    None => {panic!("read instrs none")}
                    Some(p) => {
                        if p.1 != opcodes::End_ {
                            panic!("invalid block end:{:?}",p.1)
                        }
                        Some(instruction::ArgsEnum::BlockArgs(instruction::BlockArgs{
                            bt: Some(i),
                            instrs: Some(p.0)
                        }))
                    }
                }
            }
        }

    }

    pub fn read_block_type(&mut self) -> Option<i32>{

        match self.read_var_s32() {
            None => {panic!("read i32 is none")}
            Some(n) => {
                if n < 0 {
                    match n {
                        module::BLOCK_TYPE_I32|
                        module::BLOCK_TYPE_I64|
                        module::BLOCK_TYPE_F32|
                        module::BLOCK_TYPE_F64|
                        module::BLOCK_TYPE_EMPTY => {Some(n)}
                        _ => None
                    }
                } else {
                    Some(n)
                }
            }
        }
    }

    pub fn read_if_args(&mut self) -> Option<instruction::ArgsEnum>{

        match self.read_block_type() {
            None => {panic!("read block type is none")}
            Some(i) => {
                match self.read_instructions() {
                    None => {panic!("read instrs is none")}
                    Some(p) => {
                        let mut ia = instruction::IfArgs{
                            bt: Some(i),
                            instrs1: Some(p.0),
                            instrs2: None
                        };

                        if p.1 == opcodes::End_ {
                            match self.read_instructions() {
                                None => {panic!("read instrs2 none")}
                                Some(p2) => {
                                    if p2.1 != opcodes::End_ {
                                        panic!("invalid block end:{:?}",p.1)
                                    }

                                    ia.instrs2 = Some(p2.0);
                                }
                            }
                        }
                        Some(instruction::ArgsEnum::IfArgs(ia))
                    }
                }
            }
        }

    }

    pub fn read_br_table_args(&mut self) -> Option<instruction::ArgsEnum>{
        Some(instruction::ArgsEnum::BrTableArgs(instruction::BrTableArgs{
            labels: self.read_indices(),
            default: self.read_var_u32(),
        }))
    }

    pub fn read_call_indirect_args(&mut self) -> Option<u32>{

        match self.read_var_u32() {
            None => {panic!("read var u32 none")}
            Some(n) => {
                self.read_zero();
                Some(n)
            }
        }
    }

    pub fn read_mem_arg(&mut self) -> Option<instruction::MemArg>{

        Some(instruction::MemArg{
            align: self.read_var_u32(),
            offset: self.read_var_u32(),
        })
    }

    pub fn read_zero(&mut self) -> Option<u8>{
        self.read_byte().and_then(|b|{
            if b != 0 {
                panic!("zero flag expected, got:{:?}",b)
            } else {
                Some(0)
            }
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test1(){
        use crate::binary::reader;
        let v = b"hello".to_vec();
        let mut r = reader::WasmReader{
            data: v
        };

        println!("{:?}",r.read_u32());
        println!("{:?}",r.remaining());
    }

    #[test]
    fn test2(){
        use crate::binary::{reader,module,leb128};

        let v:Vec<u8> = vec![
            0x01,
            0x02, 0x03, 0x04, 0x05,
            0x00, 0x00, 0xc0, 0x3f,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x3f,
            0xE5, 0x8E, 0x26, // https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
            0xC0, 0xBB, 0x78, // https://en.wikipedia.org/wiki/LEB128#Signed_LEB128
            0xC0, 0xBB, 0x78,
            0x03, 0x01, 0x02, 0x03,
            0x03, 0x66, 0x6f, 0x6f,
            0x80, 0x80, 0xc0, 0x0b
        ];

        let mut reader = reader::WasmReader{ data: v };


        assert_eq!(1u8,reader.read_byte().unwrap());
        assert_eq!(0x05040302,reader.read_u32().unwrap());
        assert_eq!(1.5f32,reader.read_f32().unwrap());
        assert_eq!(1.5f64,reader.read_f64().unwrap());
        assert_eq!(624485u32,reader.read_var_u32().unwrap());
        assert_eq!(-123456i32,reader.read_var_s32().unwrap());
        assert_eq!(-123456i64,reader.read_var_s64().unwrap());
        assert_eq!(vec![0x01,0x02,0x03],reader.read_bytes().unwrap());
        assert_eq!("foo".to_string(),reader.read_name().unwrap());
        assert_eq!(624485u32,reader.read_var_u32().unwrap());
    }
}
