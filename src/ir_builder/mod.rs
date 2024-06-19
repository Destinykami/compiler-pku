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
        curr_type:None,
        tmp_constants: None,
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    println!("{:#?}",my_ir_generator_info.curr_symbols);
    Ok(program)
}

pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>, // Current block
    curr_func: Option<Function>,    // Current function
    curr_value:Option<Value>,       // Current return Value
    curr_symbols:HashMap<String,SymbolsEntry>, //符号表
    curr_type:Option<Typekind>,
    tmp_constants: Option<(i32, i32)>, // Temporary constant
}
#[derive(Debug)]
//符号表的类型 const/var
pub enum SymbolsEntry{
    Variable(Typekind,Option<Value>),
    Const(Typekind,i32),
}
//类型
#[derive(Clone, Copy,PartialEq,Debug)]
pub enum Typekind {
    Const,
    Variable,
}