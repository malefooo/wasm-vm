use std::any::type_name;

pub fn judge_type<T>(_:T) ->String{
    println!("{:?}",type_name::<T>());
    format!("{:?}",type_name::<T>())
}


#[cfg(test)]
mod test{

    use crate::binary::instruction::ArgsEnum;
    use crate::utils::judge_type;
    use std::ptr::eq;

    #[test]
    pub fn test1(){
        let v1 = ArgsEnum::I32(32);
        let v2 = ArgsEnum::F32(3.1);

        assert!(!v1.eq(&v2));
    }
}