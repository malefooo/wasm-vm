pub mod module;
pub mod leb128;
pub mod reader;
pub mod opcodes;
pub mod instruction;

pub fn init(){
    opcodes::init();
}