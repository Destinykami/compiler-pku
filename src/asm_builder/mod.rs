use koopa::ir::Program;


mod asm_builder;
use asm_builder::GenerateAsm;
pub fn generate_riscv_asm(mem_ir:&Program)->Result<Vec<String>, String>{
    let mut asm_code=mem_ir.generate()?;
    Ok(asm_code)
}