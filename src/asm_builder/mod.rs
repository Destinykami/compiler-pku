use std::fs::File;

use koopa::ir::{Program, Value};


mod asm_builder;
use asm_builder::GenerateAsm;
//寄存器列表
const REGISTER_NAMES: [&str; 32] = [
    "x0", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5",
    "t6",
];
//可用于临时寄存器的列表
const REGISTER_FOR_TEMP: [usize; 15] = [5, 6, 7, 28, 29, 30, 31, 10, 11, 12, 13, 14, 15, 16, 17];
pub fn generate_riscv_asm(mem_ir:&Program,output_file:File)->Result<(), String>{
    let mut asm_info=GenerateAsmInfo{
        register_user:[None;32],
        output_file,
    };
    mem_ir.generate(&mut asm_info)?;
    Ok(())
}
pub struct GenerateAsmInfo{
    register_user:[Option<Value>;32],
    output_file: File,
}
impl GenerateAsmInfo {
    //寻找一个可用的寄存器
    fn get_available_register(&self)->usize{
        for i in REGISTER_FOR_TEMP{
            match self.register_user[i] {
                None=>return i,
                Some(_) => continue,
            }
        }
        panic!();  //寄存器用完了
    }
    //通过Value寻找寄存器
    fn find_using_register(&self,value:Value)->Option<usize>{
        for i in 0..32{
            if let Some(v)=self.register_user[i]{
                if v==value{
                    return Some(i);
                }
            }
        }
        None
    }
    //为寄存器分配使用者Value
    fn allocate_register(&mut self,value:Value)->usize{
        let reg=self.get_available_register();
        self.register_user[reg]=Some(value);
        reg
    }
    //释放寄存器
    fn free_register(&mut self,reg_id:usize){
        self.register_user[reg_id]=None;
    }
}