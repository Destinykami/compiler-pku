//! This module is the frontend of my compiler.
//! It converts the C code into Koopa IR.

mod ir_builder;
use std::collections::HashMap;

use crate::ast::statements::*;
use ir_builder::Buildable;
use koopa::ir::entities::{BasicBlock, Function}; // Koopa IR builder
use koopa::ir::{Program, Value}; // All the symbol defined in the AST

pub fn generate_ir(comp_unit: &CompUnit) -> Result<Program, String> {
    let mut program = Program::new();
    let mut my_ir_generator_info = MyIRGeneratorInfo {
        curr_block: None,
        curr_func: None,
        curr_value:None,
        curr_symbols:HashMap::new(),
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    println!("{:#?}",my_ir_generator_info.curr_symbols);
    Ok(program)
}

pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>, // Current block
    curr_func: Option<Function>,    // Current function
    curr_value:Option<Value>,       // Current return Value
    curr_symbols:HashMap<String,Option<Value>>, //符号表
}