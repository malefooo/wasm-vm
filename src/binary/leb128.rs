use crate::common::common_error::CommonError;
use anyhow::Context;

/// 解码是高左低右
pub fn decode_var_uint(data:&[u8],size:u8) -> anyhow::Result<(u64,usize),&'static str>{
    let mut result = 0u64;
    let mut count= 0u8;

    for datum in data {
        let d = *datum;
        if count == size/7 {
            //如果最后一个字节(32位是5,64位是9)的最高单字节不是0,报错
            if d & 0x80 != 0 {
                return Err("leb128 err");
            }

        }
        //data中,下标小的是低位,因此越往后的越要左移下标*7位
        result = result | ((d as u64) & 0x7f)<<(count * 7);
        count=count+1;
        if d&0x80 == 0 {
            break;
        }

    }

    Ok((result,count as usize))
}

pub fn decode_var_int(data:&[u8], size:u8) -> anyhow::Result<(i64,usize),&'static str>{
    let mut result = 0i64;
    let mut count= 0u8;

    for datum in data {
        let d = *datum;
        if count == size/7 {
            //如果最后一个字节(32位是5,64位是9)的最高单字节不是0,报错
            if d & 0x80 != 0 {
                return Err("leb128 err");
            }

        }
        //data中,下标小的是低位,因此越往后的越要左移下标*7位
        result = result | ((d as i64) & 0x7f)<<(count * 7);
        count=count+1;
        if datum&0x80 == 0 {
            //负数
            if ((count-1)*7<size) && (d&0x40 != 0) {
                result = result | (-1<<(count * 7));
            }
            break;
        }
    }

    Ok((result,count as usize))
}

#[cfg(test)]
mod test {
    #[test]
    fn test1(){
        let v:Vec<u8> = vec![144,78];
        let o = crate::binary::leb128::decode_var_uint(v.as_slice(),32);
        println!("{:?}",o);
    }

    #[test]
    fn test2(){
        let v:Vec<u8> = vec![0x98,0x78];
        let o = crate::binary::leb128::decode_var_int(v.as_slice(),32);
        println!("{:?}",o);
    }
}