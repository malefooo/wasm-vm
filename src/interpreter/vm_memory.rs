
use crate::binary::module;

#[derive(Debug,Clone)]
pub struct Memory{
    pub _type:module::MemType,
    pub data:Vec<u8>
}

impl Memory{
    pub fn new(mt:module::MemType) -> Memory{
        let mut v:Vec<u8> = Vec::new();
        v.resize((mt.min.unwrap() as usize * module::PAGE_SIZE) as usize,0);
        Memory{
            _type: mt,
            data: v,
        }
    }

    /// 计算页数,数组长度/一页长度
    pub fn size(&self) -> usize{
        self.data.len() / module::PAGE_SIZE
    }

    /// 增长页数
    pub fn grow(&mut self,n:usize) -> usize{
        let old_size = self.size();
        if n == 0 {
            return old_size
        }
        let mut max_page_count = module::MAX_PAGE_COUNT;
        self._type.max.and_then(|v|{
            if v >= 0 {
                max_page_count = v as usize;
            }
            Some(())
        }).or_else(||{
            println!("mem max is none");
            None
        });

        if old_size + n > max_page_count {
            return 0xFFFFFFFF
        }
        let mut new_v:Vec<u8> = Vec::new();
        new_v.resize((old_size+n)*module::PAGE_SIZE,0);
        let (left,right) = new_v.split_at_mut(self.data.len());
        left.clone_from_slice(self.data.as_slice());
        self.data = new_v;
        old_size
    }

    /// 读数据
    pub fn read(&mut self, offset:usize, buf: &mut [u8]){
        self.check_offset(offset,buf.len());
        let (left,right) = self.data.split_at_mut(offset);
        let (r_left,r_right) = right.split_at_mut(buf.len());
        buf.clone_from_slice(r_left)
    }

    /// 写数据
    pub fn write(&mut self,offset:usize,data:&[u8]){
        self.check_offset(offset,data.len());
        let (left,right) = self.data.split_at_mut(offset);
        let (r_left,r_right) = right.split_at_mut(data.len());
        r_left.clone_from_slice(data);
    }

    /// 校验是否越界
    fn check_offset(&mut self,offset:usize,length:usize){
        if self.data.len() - length < offset {
            panic!("errMemOutOfBounds")
        }
    }
}