//遍历内存形式的IR,解析得到汇编代码

use koopa::ir::{FunctionData, Value, ValueKind};

use crate::asm_builder::REGISTER_NAMES;

use super::GenerateAsmInfo;
use std::io::Write;
// 根据内存形式 Koopa IR 生成汇编
pub trait GenerateAsm {
    fn generate(&self, asm_info: &mut GenerateAsmInfo) -> Result<(), String>;
}

impl GenerateAsm for koopa::ir::Program {
    fn generate(&self, asm_info: &mut GenerateAsmInfo) -> Result<(), String> {
        let mut asm_codes = Vec::<String>::new();
        asm_codes.push(String::from("  .text"));
        //生成函数的汇编代码
        for &func in self.func_layout() {
            self.func(func).generate(asm_info)?;
        }
        Ok(())
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self, asm_info: &mut GenerateAsmInfo) -> Result<(), String> {
        writeln!(asm_info.output_file, "  .global {}", &self.name()[1..]).expect("Write error. ");
        writeln!(asm_info.output_file, "{}:", &self.name()[1..]).expect("Write error. ");
        for (&bb, node) in self.layout().bbs() {
            //遍历基本块列表
            for &inst in node.insts().keys() {
                //访问指令列表
                //访问指令
                let value_data = self.dfg().value(inst);
                match value_data.kind() {
                    ValueKind::Integer(int) => {
                        //处理int指令
                        todo!()
                    }
                    ValueKind::Return(ret) => {
                        //处理return
                        match ret.value() {
                            Some(ret) => {
                                let val = self.dfg().value(ret);
                                match val.kind() {
                                    ValueKind::Integer(int) => {
                                        writeln!(asm_info.output_file, "  li a0, {}", int.value())
                                            .expect("Write error. ");
                                    }
                                    ValueKind::Binary(binary)=>{
                                        writeln!(asm_info.output_file, "  mv a0, {}", REGISTER_NAMES[asm_info.find_using_register(ret).expect("No register for return value. Should never happen! ")])
                                            .expect("Write error. ");
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            None => {}
                        }
                        writeln!(asm_info.output_file, "  ret").expect("Write error. ");
                    }
                    ValueKind::Binary(binary) => {
                        let reg1 = get_reg(self, binary.lhs(), asm_info)?;
                        let reg2 = get_reg(self, binary.rhs(), asm_info)?;
                        asm_info.free_register(reg1);
                        asm_info.free_register(reg2);  //让结果可以复用上面的寄存器
                        let reg_ans=asm_info.allocate_register(inst); //为指令的返回值分配寄存器   为什么不用get_reg?
                        match binary.op() {
                            koopa::ir::BinaryOp::NotEq => todo!(),
                            koopa::ir::BinaryOp::Eq => {
                                //你也许会注意到, 如果按照一条指令的结果占用一个临时寄存器的目标代码生成思路, 在表达式足够复杂的情况下,
                                //所有的临时寄存器很快就会被用完. 本章出现的测试用例中会避免出现这种情况,
                                //同时, 你可以自行思考: 用何种方式可以缓解这个问题. 在 Lv4 中, 我们会给出一种一劳永逸的思路来解决这个问题.
                                writeln!(asm_info.output_file,"  xor   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                writeln!(asm_info.output_file,"  seqz  {},{}", REGISTER_NAMES[reg_ans],REGISTER_NAMES[reg_ans]).expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Gt => {
                                //sgt是一个伪指令,也就是说, 这条指令并不真实存在, 而是用其他指令实现的.
                                //sgt t0, t1, t2 (判断 t1 的值是否大于 t2 的值) 是怎么实现的?
                                // = slt t0,t2,t1
                                
                                writeln!(asm_info.output_file,"  sgt   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                writeln!(asm_info.output_file,"  snez   {},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans]).expect("Write error. ");
                                
                            },
                            koopa::ir::BinaryOp::Lt => {
                                //slt t0, t1, t2 指令的含义是, 判断寄存器 t1 的值是否小于 t2 的值, 并将结果 (0 或 1) 写入 t0 寄存器. 
                                writeln!(asm_info.output_file,"  slt   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                writeln!(asm_info.output_file,"  snez   {},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans]).expect("Write error. ");
                                
                            },
                            koopa::ir::BinaryOp::Ge => {
                                //判断大于等于的原理是什么? => 判断是否小于后面，取反
                                writeln!(asm_info.output_file,"  sgt   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                writeln!(asm_info.output_file,"  seqz   {},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans]).expect("Write error. ");
                            },
                            koopa::ir::BinaryOp::Le => {
                                //判断小于等于的原理是什么? => 判断是否大于后面，取反
                                writeln!(asm_info.output_file,"  sgt   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                writeln!(asm_info.output_file,"  seqz   {},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans]).expect("Write error. ");

                            },
                            koopa::ir::BinaryOp::Add => {
                                writeln!(asm_info.output_file,"  add   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                            },
                            koopa::ir::BinaryOp::Sub => {
                                writeln!(asm_info.output_file,"  sub   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                            },
                            koopa::ir::BinaryOp::Mul => {
                                writeln!(asm_info.output_file,"  mul   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                
                            },
                            koopa::ir::BinaryOp::Div => {
                                writeln!(asm_info.output_file,"  div   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                            },
                            koopa::ir::BinaryOp::Mod => {
                                writeln!(asm_info.output_file,"  rem   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                            },
                            koopa::ir::BinaryOp::And => {
                                writeln!(asm_info.output_file,"  and   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                
                            },
                            koopa::ir::BinaryOp::Or => {
                                writeln!(asm_info.output_file,"  or   {},{},{}",REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1],REGISTER_NAMES[reg2]).expect("Write error. ");
                                
                            },
                            koopa::ir::BinaryOp::Xor => todo!(),
                            koopa::ir::BinaryOp::Shl => todo!(),
                            koopa::ir::BinaryOp::Shr => todo!(),
                            koopa::ir::BinaryOp::Sar => todo!(),
                        }
                    }
                    // 其他种类暂时遇不到
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}


/// -.-
/// Given a component of a binary op (e.g. an Integer or answer of another binary op),
/// get its string representation in assembly code.
///
/// e.g. %1 = sub 0, %0, the lhs is converted into "0",
/// and the rhs is converted into "t0" (its register).
fn get_reg(
    fd: &FunctionData,
    value: Value,
    asm_info: &mut GenerateAsmInfo,
) -> Result<usize, String> {
    match fd.dfg().value(value).kind() {
        ValueKind::Integer(int) => {
            // Allocate a new register for the Integer.
            if int.value()==0 {
                Ok(0) // Register x0(id=0) is always 0. 
            }
            else {
                let reg = asm_info.allocate_register(value);
                writeln!(asm_info.output_file, "  li   {}, {}", REGISTER_NAMES[reg], int.value()).expect("Write error. ");
                Ok(reg)
            }
        }
        ValueKind::Binary(_) => {
            // Use the register allocated for this expression.
            let reg = asm_info.find_using_register(value).expect(
                    "No register for expression. This should not happen. \n",
            );
            Ok(reg)
        }
        value_kind => Err(format!("Wrong type in LHS: {:?}", value_kind)),
    }
}