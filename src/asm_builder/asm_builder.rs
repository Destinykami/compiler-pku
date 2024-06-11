//遍历内存形式的IR,解析得到汇编代码

use koopa::ir::ValueKind;

// 根据内存形式 Koopa IR 生成汇编
pub trait GenerateAsm {
    fn generate(&self)->Result<Vec<String>,String>;
}

impl GenerateAsm for koopa::ir::Program {
    fn generate(&self) ->Result<Vec<String>,String>{
        let mut asm_codes=Vec::<String>::new();
        asm_codes.push(String::from("  .text"));
        //生成函数的汇编代码
        for &func in self.func_layout() {   //循环遍历函数列表
            asm_codes.extend(self.func(func).generate()?);
        }
        Ok(asm_codes)
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self)->Result<Vec<String>,String> {
        let mut asm_codes=Vec::<String>::new();
        asm_codes.push(format!("  .global {}",&self.name()[1..]).to_string()); //.global {fun_name}
        asm_codes.push(format!("{}:",&self.name()[1..]).to_string()); //{fun_name}]:
        for (&bb,node) in self.layout().bbs(){
            //遍历基本块列表
            for &inst in node.insts().keys(){
                //访问指令列表
                //访问指令
                let value_data=self.dfg().value(inst); 
                match value_data.kind(){
                    ValueKind::Integer(int)=>{
                        //处理int指令
                        todo!()
                    }
                    ValueKind::Return(ret)=>{
                        //处理return
                        match ret.value(){
                            Some(ret)=>{
                                let val=self.dfg().value(ret);
                                match val.kind(){
                                    ValueKind::Integer(int)=>{
                                        asm_codes.push(format!("  li a0, {}",int.value()).to_string());
                                    },
                                    ValueKind::ZeroInit(_) => todo!(),
                                    ValueKind::Undef(_) => todo!(),
                                    ValueKind::Aggregate(_) => todo!(),
                                    ValueKind::FuncArgRef(_) => todo!(),
                                    ValueKind::BlockArgRef(_) => todo!(),
                                    ValueKind::Alloc(_) => todo!(),
                                    ValueKind::GlobalAlloc(_) => todo!(),
                                    ValueKind::Load(_) => todo!(),
                                    ValueKind::Store(_) => todo!(),
                                    ValueKind::GetPtr(_) => todo!(),
                                    ValueKind::GetElemPtr(_) => todo!(),
                                    ValueKind::Binary(_) => todo!(),
                                    ValueKind::Branch(_) => todo!(),
                                    ValueKind::Jump(_) => todo!(),
                                    ValueKind::Call(_) => todo!(),
                                    ValueKind::Return(_) => todo!(),
                                    
                                }

                            }
                            None => {},
                        }
                        asm_codes.push(String::from("  ret"));
                    }
                    // 其他种类暂时遇不到
                    _=>unreachable!(),
                }
            }
        }
        Ok(asm_codes)
    }
}
