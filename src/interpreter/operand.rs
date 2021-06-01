use crate::binary::instruction::ArgsEnum;

#[derive(Debug,Clone)]
pub struct OperandStack{
    slots:Vec<ArgsEnum>
}

pub fn new() -> OperandStack{
    let v:Vec<ArgsEnum> = Vec::new();
    OperandStack{
        slots: v
    }
}

impl OperandStack {

    pub fn push(&mut self,val:ArgsEnum){
        self.slots.push(val);
    }

    pub fn push_i8(&mut self, val:i8){
        self.slots.push(ArgsEnum::I8(val));
    }

    pub fn pop_i8(&mut self) -> Option<i8>{
        self.slots.pop().and_then(|e|Some(e.get_i8()))
    }

    pub fn push_i16(&mut self, val:i16){
        self.slots.push(ArgsEnum::I16(val));
    }

    pub fn pop_i16(&mut self)->Option<i16>{
        self.slots.pop().and_then(|e|Some(e.get_i16()))
    }

    pub fn push_u16(&mut self,val:u16){
        self.slots.push(ArgsEnum::U16(val));
    }

    pub fn pop_u16(&mut self)->Option<u16>{
        self.slots.pop().and_then(|e|Some(e.get_u16()))
    }

    pub fn push_u64(&mut self, val:u64){
        self.slots.push(ArgsEnum::U64(val));
    }

    pub fn pop_u64(&mut self) -> Option<u64>{
        self.slots.pop().and_then(|e|Some(e.get_u64()))
    }

    pub fn push_s64(&mut self, val:i64){
        self.slots.push(ArgsEnum::I64(val))
    }

    pub fn pop_s64(&mut self)->Option<i64>{
        self.slots.pop().and_then(|e| Some(e.get_i64()))
    }

    pub fn push_u32(&mut self,val:u32){
        self.slots.push(ArgsEnum::U32(val))
    }

    pub fn pop_u32(&mut self) -> Option<u32>{
        self.slots.pop().and_then(|e|Some(e.get_u32()))
    }

    pub fn push_s32(&mut self ,val:i32){
        self.slots.push(ArgsEnum::I32(val))
    }

    pub fn pop_s32(&mut self) -> Option<i32>{
        self.slots.pop().and_then(|e|Some(e.get_i32()))
    }

    pub fn push_f64(&mut self,val:f64){
        self.slots.push(ArgsEnum::F64(val))
    }

    pub fn pop_f64(&mut self)->Option<f64>{
        self.slots.pop().and_then(|e|Some(e.get_f64()))
    }

    pub fn push_f32(&mut self,val:f32){
        self.slots.push(ArgsEnum::F32(val))
    }

    pub fn pop_f32(&mut self) -> Option<f32>{
        self.slots.pop().and_then(|e| Some(e.get_f32()))
    }

    pub fn push_bool(&mut self,val:i32){
        if val == 1 {
            self.slots.push(ArgsEnum::Bool(true));
        } else {
            self.slots.push(ArgsEnum::Bool(false));
        }
    }

    pub fn pop_bool(&mut self)->Option<bool>{
        self.slots.pop().and_then(|e|Some(e.get_bool()))
    }

    pub fn pop(&mut self) -> Option<ArgsEnum>{
        self.slots.pop()
    }
}

#[cfg(test)]
mod test{

    #[test]
    fn test1(){
        use crate::interpreter::operand;
        let mut stack = operand::new();

        stack.push_u64(64_u64);
        assert_eq!(stack.pop_u64(),Some(64_u64));

        stack.push_f64(0.64_f64);
        assert_eq!(stack.pop_f64(),Some(0.64_f64));

        stack.push_u32(32_u32);
        assert_eq!(stack.pop_u32(),Some(32_u32));

        stack.push_f32(0.32_f32);
        assert_eq!(stack.pop_f32(),Some(0.32_f32));

        stack.push_s32(31_i32);
        assert_eq!(stack.pop_s32(),Some(31_i32));

        stack.push_s64(61_i64);
        assert_eq!(stack.pop_s64(),Some(61_i64));

        stack.push_bool(1);
        assert_eq!(stack.pop_bool(),Some(true));
    }
}