//! Build a single component into Koopa IR.

use crate::ast::ast::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::MyIRGeneratorInfo;

pub trait Buildable {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String>;
}

impl Buildable for CompUnit {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        self.func_def.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl Buildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let return_type = match self.return_type.type_name.as_str() {
            "int" => Ok(Type::get_i32()),
            _ => Err("Wrong return type"),
        };
        //dbg!("Building function", &self);

        //Create a new program for current program
        let func = program.new_func(FunctionData::with_param_names(
            "@".to_string() + self.func_id.as_str(),
            vec![],
            return_type.expect("Error creating new function"),
        ));
        //println!("{}", self.func_id.as_str());  //main
        let func_data = program.func_mut(func);
        let new_block = func_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".to_string()));
        func_data.layout_mut().bbs_mut().extend([new_block]);
        my_ir_generator_info.curr_block = Some(new_block);
        my_ir_generator_info.curr_func = Some(func);
        self.block.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl Buildable for Block {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        self.stmt.build(program, my_ir_generator_info)?;
        Ok(())
    }
}
impl Buildable for Exp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Exp::UnaryExp(unary_exp) => unary_exp.build(program, my_ir_generator_info),
        }
    }
}
impl Buildable for UnaryExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            UnaryExp::PrimaryExp(primary_exp) => primary_exp.build(program, my_ir_generator_info),
            UnaryExp::PlusUnaryExp(plus_unary_exp) => {
                plus_unary_exp.build(program, my_ir_generator_info)
            }
            UnaryExp::MinusUnaryExp(unary_exp) => build_binary_from_buildables(
                &Number::IntConst(0),
                &**unary_exp,   //what can i say
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Sub,
            ),
            UnaryExp::NotUnaryExp(unary_exp) => build_binary_from_buildables(
                &**unary_exp,
                &Number::IntConst(0),

                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Eq,
            ),
        }
    }
}
impl Buildable for Stmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match &self {
            Stmt::ReturnStmt(exp) => {
                exp.build(program, my_ir_generator_info)?;
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                let return_stmt = curr_func_data
                    .dfg_mut()
                    .new_value()
                    .ret(my_ir_generator_info.curr_value);
                curr_func_data
                    .layout_mut()
                    .bb_mut(my_ir_generator_info.curr_block.unwrap())
                    .insts_mut()
                    .extend([return_stmt]);
            }
        }
        Ok(())
    }
}
impl Buildable for PrimaryExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            PrimaryExp::BracedExp(exp) => exp.build(program, my_ir_generator_info),
            PrimaryExp::Number(number) => number.build(program, my_ir_generator_info),
        }
    }
}

impl Buildable for Number {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Number::IntConst(int) => {
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                my_ir_generator_info.curr_value =
                    Some(curr_func_data.dfg_mut().new_value().integer(*int));
                Ok(())
            }
        }
    }
}

//不懂-。-
fn build_binary_from_buildables(
    first_exp: &dyn Buildable,
    second_exp: &dyn Buildable,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<(), String> {
    first_exp.build(program, my_ir_generator_info)?;
    let first_value = my_ir_generator_info
        .curr_value
        .expect("No curr_value. Should not happen. ");
    second_exp.build(program, my_ir_generator_info)?;
    let second_value = my_ir_generator_info
        .curr_value
        .expect("No curr_value. Should not happen. ");
    build_binary_from_values(
        first_value,
        second_value,
        program,
        my_ir_generator_info,
        binary_op,
    )
}

//用二元表达式表示一元表达式
fn build_binary_from_values(
    first_value: koopa::ir::Value,
    second_value: koopa::ir::Value,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<(), String> {
    let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
    let new_value =
        curr_func_data
            .dfg_mut()
            .new_value()
            .binary(binary_op, first_value, second_value);
    curr_func_data
        .layout_mut()
        .bb_mut(my_ir_generator_info.curr_block.unwrap())
        .insts_mut()
        .extend([new_value]);
    my_ir_generator_info.curr_value = Some(new_value);
    Ok(())
}
