use crate::binary::instruction;

/**
机器码映射

类型段格式:
01     19       05         60   01      7F   00         60 00 00 60 02 7F 7F 01 7F 60 01 7F 01 7F 01 7F 60 03 7F 7F 7F 00
|      |        |          |    |       |    |          |
类型段  25个字节  5条类型数据  方法  1个参数  i32  返回none   同理映射

导入段格式:
tag:0-函数/1-表/2-内存/3-全局变量
02     12       01      03         65 6E 76    0A         70 72 69 6E 74 5F 63 68 61 72  00    00
|      |        |       |          |     |     |          |                          |   |     |
导入段  18个字节  1条数据  模块名3字节    'env'     成员名10字节        'print'                 tag   签名索引:0

函数段格式:
03     0C      0B       01  02  02  02  02  03  02  04  02  04  03
|      |       |
函数段  12个字节 11个数据

表段:
tag:0-只有下限/1-上下限都有
04    05      01       70              01    01    01
|     |       |        |               |     |     |
表段   5个字节  1个数据  元素类型:函数引用  tag   下限   上限

内存段:
tag:0-只有下限/1-上下限都有
05      03       01      00    11
|       |        |       |     |
内存段   3个字节   1个数据  tag  下限

全局段:
06     19       03      7F   01    41  80  80  C0  00  0B  7F  00    41  8E  80  C0  00 0B 7F   00    41 8E 80 C0 00 0B
|      |        |       |    |     |                       |   |     |                     |    |     |
全局段  25个字节  3个数据  i32  可变  操作数                    i32 不可变 操作数                 i32  不可变 操作数

导出段:
tag:0-函数/1-表/2-内存/3-全局变量
07     2C      04      06      6D 65 6D 6F 72 79 02  00
|      |       |       |       |              |  |   |
导出段  44个字段 4个数据  名字6字节     'memory'     tag 索引
0A        5F 5F 64 61 74 61 5F 65 6E 64 03  01
|         |                           | |   |
名字10字节     '__data_end'              tag 索引
0B        5F 5F 68 65 61 70 5F 62 61 73 65 03  02
|         |                              | |   |
名字11字节          '__heap_base'           tag 索引
04       6D 61 69 6E 00   01
|        |        |  |    |
名字4字节   'main'    tag  索引

代码段:
0A     D6 08      0B       EA 01   01            16       7F    23 80 80 80 80 00 21 00 41 20 21 01 20 00 20 01 6B 21 02 20 02 20 02 24 80 80 80 80 00 41 80
|      |  |       |        |  |    |             |        |
代码段  1110个字节  11个数据  234字节  1个局部变量组   22个变量  i32

数据段:
0B     17       01     00     41   80 80 C0 00 0B  0E              48 65 6C 6C 6F 2C 20 57 6F 72 6C 64 21 0A
|      |        |      |      |    |            |  |
数据段  23个字节  1个数据 索引0  偏移量   立即数,0B结尾   14个字节,初始数据

自定义段:
00        FC     06    04          6E 61 6D 65 01 F4
|         |      |     |           |        |
自定义段    内容892      自定义段名称    'name'
**/

/// 魔术号 常量
pub const MAGIC_NUMBER:&[u8;4] = b"\0asm";
/// 版本号 常量
pub const VERSION:&[u8;1] = b"1";
/// 导入函数,表,内存,全局变量的tag值,这个tag值是导入描述的地一个单字节
pub const IMPORT_TAG_FUNC:u8 = 0;
pub const IMPORT_TAG_TABLE:u8 = 1;
pub const IMPORT_TAG_MEM:u8 = 2;
pub const IMPORT_TAG_GLOBAL:u8 = 3;

pub const EXPORT_TAG_FUNC:u8 = 0;
pub const EXPORT_TAG_TABLE:u8 = 1;
pub const EXPORT_TAG_MEM:u8 = 2;
pub const EXPORT_TAG_GLOBAL:u8 = 3;

pub const SEC_CUSTOM_ID:u8 = 0;
pub const SEC_TYPE_ID:u8 = 1;
pub const SEC_IMPORT_ID:u8 = 2;
pub const SEC_FUNC_ID:u8 = 3;
pub const SEC_TABLE_ID:u8 = 4;
pub const SEC_MEM_ID:u8 = 5;
pub const SEC_GLOBAL_ID:u8 = 6;
pub const SEC_EXPORT_ID:u8 = 7;
pub const SEC_START_ID:u8 = 8;
pub const SEC_ELEM_ID:u8 = 9;
pub const SEC_CODE_ID:u8 = 10;
pub const SEC_DATA_ID:u8 = 11;

/// 类型
pub const VAL_TYPE_I32:u8 = 0x7f;
pub const VAL_TYPE_I64:u8 = 0x7e;
pub const VAL_TYPE_F32:u8 = 0x7d;
pub const VAL_TYPE_F64:u8 = 0x7c;

pub const BLOCK_TYPE_I32:i32 = -1;
pub const BLOCK_TYPE_I64:i32 = -2;
pub const BLOCK_TYPE_F32:i32 = -3;
pub const BLOCK_TYPE_F64:i32 = -4;
pub const BLOCK_TYPE_EMPTY:i32 = -64;



pub const MUT_CONST:u8 = 0;
pub const MUT_VAR:u8 = 1;
pub const FT_TAG:u8 = 0x60;
pub const FUNC_REF:u8 = 0x70;


///wasm二进制格式的结构提映射
#[derive(Debug,Clone)]
pub struct Module{
    pub magic:Option<u32>,//魔数
    pub version:Option<u32>,//版本号
    pub custom_secs:Option<Vec<CustomSecs>>,//自定义段
    pub type_sec:Option<Vec<FuncType>>,//函数段
    pub import_sec:Option<Vec<Import>>,//导入段
    pub func_sec:Option<Vec<TypeIdx>>,//类型段
    pub table_sec:Option<Vec<TableType>>,//标签段
    pub mem_sec:Option<Vec<MemType>>,//内存段
    pub global_sec:Option<Vec<GlobalSec>>,//全局变量段
    pub export_sec:Option<Vec<Export>>,//导出段
    pub start_sec:Option<FuncIdx>,//起始段
    pub elem_sec:Option<Vec<Elem>>,//元素段
    pub code_sec:Option<Vec<Code>>,//代码段
    pub data_sec:Option<Vec<Data>>,//数据段
}

/// 导入段
#[derive(Debug,Clone)]
pub struct Import{
    pub module:Option<String>,
    pub name:Option<String>,
    pub import_desc:Option<ImportDesc>,
}

/// 导入段描述
#[derive(Debug,Clone)]
pub struct ImportDesc{
    pub tag:Option<u8>,
    pub fun_type:Option<TypeIdx>,
    pub table:Option<TableType>,
    pub mem:Option<MemType>,
    pub global:Option<GlobalType>,
}

// #[derive(Debug,Clone)]
// pub struct Expr{}

///全局段
#[derive(Debug,Clone)]
pub struct GlobalSec {
    pub ty:Option<GlobalType>,
    pub init:Option<instruction::Expr>,
}

#[derive(Debug,Clone)]
pub struct Export {
    pub name:Option<String>,
    pub desc:Option<ExportDesc>,
}

#[derive(Debug,Clone)]
pub struct ExportDesc{
    pub tag:Option<u8>,
    pub idx:Option<u32>,
}

#[derive(Debug,Clone)]
pub struct Elem{
    pub table:Option<TableIdx>,
    pub offset:Option<instruction::Expr>,
    pub init:Option<Vec<FuncIdx>>,
}

#[derive(Debug,Clone)]
pub struct Code {
    pub locals:Option<Vec<Locals>>,
    pub expr:Option<instruction::Expr>,
}

impl Code {
    pub fn get_local_count(&self) -> Option<u32>{
        if self.locals.is_some() {
            let mut n = 0u32;
            for l in  self.locals.clone().unwrap().iter() {
                n = n + l.n.clone().unwrap();
            }
            Some(n)
        } else {
            None
        }
    }
}

#[derive(Debug,Clone)]
pub struct Locals {
    pub n:Option<u32>,
    pub ty:Option<u8>,
}

#[derive(Debug,Clone)]
pub struct Data{
    pub mem:Option<MemIdx>,
    pub offset:Option<instruction::Expr>,
    pub init:Option<Vec<u8>>,
}

#[derive(Debug,Clone)]
pub struct CustomSecs{
    pub name:Option<String>,
    pub bytes:Vec<u8>,
}

/// 索引空间
/// TypeIdx 类型索引 例如:5个函数即取值0~4
/// FuncIdx 函数索引 例如:内部2个函数,外部3个函数,即取值0~4
/// TableIdx 表索引 目前限制只能有一个,取值即0
/// MemIdx 内存索引 目前限制只能有一个,取值即0
/// GlobalIdx 全局变量索引 外部全局变量+内部全局变量
/// LocalIdx 局部变量索引 函数接受的参数+函数内部局部变量
/// LabelIdx 跳表标签索引 每个函数有自己的跳表标签
pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type LocalIdx = u32;
pub type LabelIdx = u32;



///函数类型
/// param_types 参数集合
/// result_types 返回值集合
#[derive(Debug,Clone)]
pub struct FuncType {
    pub tag:Option<u8>,
    pub param_types:Option<Vec<u8>>,
    pub result_types:Option<Vec<u8>>,
}

/// 限制类型
/// min 下限
/// max 上限
#[derive(Debug,Clone)]
pub struct Limits {
    pub tag:Option<u8>,
    pub min:Option<u32>,
    pub max:Option<u32>,
}

/// 内存类型
/// 只需描述内存的限制类型,所以直接就是Limits
pub type MemType = Limits;

/// 表类型
/// elem_type目前只能是函数引用,即0x70
#[derive(Debug,Clone)]
pub struct TableType {
    pub elem_type:Option<u8>,
    pub limits:Option<Limits>,
}


#[derive(Debug,Clone)]
pub struct GlobalType {
    pub val_type:Option<u8>,
    pub m:Option<u8>,
}

#[cfg(test)]
mod test{

    #[test]
    pub fn test1(){
        use crate::binary::{reader,module,leb128};
        crate::binary::init();
        let r = reader::decode_file("./hw_rust.wasm".to_string());
        println!("{:?}",r);
    }
}