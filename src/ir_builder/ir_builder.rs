//! Build a single component into Koopa IR.

use crate::ast::{exp::*, statements::*};
use koopa::ir::{builder_traits::*, BinaryOp, FunctionData, Program, Type};
use crate::ir_builder::Typekind::*;
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
        match self {
            Block::Block(block_items) =>{
                for stmt in block_items{
                    stmt.build(program, my_ir_generator_info)?
                }
            },
        }
        Ok(())
    }
}
impl Buildable for BlockItem {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            BlockItem::Decl(decl) => decl.build(program, my_ir_generator_info),
            BlockItem::Stmt(stmt) => stmt.build(program, my_ir_generator_info),
        }
    }
}
impl Buildable for Decl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            Decl::ConstDecl(const_decl) => const_decl.build(program, my_ir_generator_info),
            Decl::VarDecl(var_decl) => var_decl.build(program, my_ir_generator_info),
        }
    }
}
impl Buildable for ConstDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            ConstDef::ConstDef(ident, const_initval) => {
                const_initval.build(program, my_ir_generator_info)?;
                let (ans,_) =my_ir_generator_info.tmp_constants.unwrap();
                my_ir_generator_info.tmp_constants=None;
                my_ir_generator_info.curr_symbols.insert(
                    ident.content.clone(), 
                    super::SymbolsEntry::Const(Const, ans)
                );

            },
        }
        Ok(())
    }
}
impl Buildable for ConstInitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            ConstInitVal::ConstExp(exp) => {
                exp.build(program, my_ir_generator_info)
            },
        }
    }
}
impl Buildable for ConstExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            ConstExp::Exp(exp) => exp.build(program, my_ir_generator_info),
        }
    }
}
impl Buildable for ConstDecl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            ConstDecl::ConstDecl(type_name, const_defs) => {
                //const int a=1,b=1;
                for const_def in const_defs{
                    //开始进行常量计算
                    my_ir_generator_info.tmp_constants=Some((520,1314));
                    const_def.build(program, my_ir_generator_info)?
                }
            },
        }
        Ok(())
    }
}
impl Buildable for VarDecl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            VarDecl::VarDecl(type_name, insides_def) => {
                for inside in insides_def{
                    inside.build(program, my_ir_generator_info);
                }
            },
        }
        Ok(())
    }
}
impl Buildable for VarDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            VarDef::VarDef(ident, initval) => {
                //定义变量的同时定义值
                initval.build(program, my_ir_generator_info)?;
                my_ir_generator_info.curr_symbols.insert(
                    ident.content.clone(), 
                    super::SymbolsEntry::Variable(Variable, my_ir_generator_info.curr_value));
            },
            VarDef::IDENT(ident) => {
                //定义变量，但不定义初始值
                todo!();
                //my_ir_generator_info.curr_symbols.insert(ident.content.clone(), Some(Value(0)));
            },
        }
        Ok(())
    }
}
impl Buildable for InitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            InitVal::Exp(exp) => exp.build(program, my_ir_generator_info),
        }
    }
}
impl Buildable for Exp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Exp::LOrExp(lor_exp) => lor_exp.build(program, my_ir_generator_info),
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
                BinaryOp::Sub,
            ),
            UnaryExp::NotUnaryExp(unary_exp) => build_binary_from_buildables(
                &**unary_exp,
                &Number::IntConst(0),
                program,
                my_ir_generator_info,
                BinaryOp::Eq,
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
            Stmt::AssignStmt(lval, exp) => {
                //为变量赋值,如果左侧为常量则报错
                lval.build(program, my_ir_generator_info)?;
                let lval_kind=my_ir_generator_info.curr_type;
                if lval_kind==Some(Const){
                    println!("Const cannot be assigned!");
                    unreachable!();
                }
                else{
                    exp.build(program, my_ir_generator_info)?;

                }
            },
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
            PrimaryExp::LVal(lval) => lval.build(program, my_ir_generator_info),
        }
    }
}
impl Buildable for LVal{
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            //在遇到 LVal 时, 你应该从符号表中查询这个符号的值, 然后用查到的结果作为常量求值/IR 生成的结果
            LVal::IDENT(ident) => 
                match my_ir_generator_info.curr_symbols.get(&ident.content).unwrap() {
                    crate::ir_builder::SymbolsEntry::Variable(_, _) => todo!(),
                    crate::ir_builder::SymbolsEntry::Const(type_name, val) => {
                        if let Some(_) =my_ir_generator_info.tmp_constants{
                            my_ir_generator_info.tmp_constants=Some((*val,123456));
                            return Ok(());
                        }
                        my_ir_generator_info.curr_value=Some(
                            program
                                .func_mut(my_ir_generator_info.curr_func.unwrap())
                                .dfg_mut()
                                .new_value()
                                .integer(*val),
                        );
                        return Ok(());
                    },
            },
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
                if let Some(_) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    my_ir_generator_info.tmp_constants = Some((*int, 233333));
                    return Ok(());
                }
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                my_ir_generator_info.curr_value =
                    Some(curr_func_data.dfg_mut().new_value().integer(*int));
                Ok(())
            }
        }
    }
}
impl Buildable for AddExp {
    fn build(
            &self,
            program: &mut Program,
            my_ir_generator_info: &mut MyIRGeneratorInfo,
        ) -> Result<(), String> {
        match self{
            AddExp::BinaryAddExp(first_exp,second_exp)=>build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Add),
            AddExp::MulExp(exp) => {
                exp.build(program, my_ir_generator_info)
            },
            AddExp::BinarySubExp(first_exp,second_exp)=>build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Sub),

        }
    }
}
impl Buildable for MulExp {
    fn build(
            &self,
            program: &mut Program,
            my_ir_generator_info: &mut MyIRGeneratorInfo,
        ) -> Result<(), String> {
        match self {
            MulExp::UnaryExp(exp)=>{
                exp.build(program, my_ir_generator_info)
            }
            MulExp::BinaryDivExp(first_exp,second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Div),
            MulExp::BinaryMulExp(first_exp,second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Mul),
            MulExp::BinaryModExp(first_exp,second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Mod),

        }
    }
}
impl Buildable for RelExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            RelExp::AddExp(exp) => exp.build(program, my_ir_generator_info),
            RelExp::BinaryLtRelExp(first_exp, second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Lt),
            RelExp::BinaryGtRelExp(first_exp, second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Gt),
            RelExp::BinaryLeRelExp(first_exp, second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Le),
            RelExp::BinaryGeRelExp(first_exp, second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Ge),
        }
    }
}
impl Buildable for EqExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            EqExp::RelExp(exp) => exp.build(program, my_ir_generator_info),
            EqExp::BinaryEqExp(first_exp, second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::Eq),
            EqExp::BinaryNotEqExp(first_exp, second_exp) => build_binary_from_buildables(&**first_exp, second_exp, program, my_ir_generator_info, BinaryOp::NotEq),
        }
    }
}
impl Buildable for LAndExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            LAndExp::EqExp(exp) => exp.build(program, my_ir_generator_info),
            LAndExp::BinaryLAndExp(first_exp, second_exp) => {
                build_binary_from_buildables(
                    &**first_exp,
                    &Number::IntConst(0),
                    program,
                    my_ir_generator_info,
                    BinaryOp::NotEq,
                )?;
                let mut tmp1 = 0;
                if let Some((tmp, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    tmp1 = tmp;
                }
                let bool1 = my_ir_generator_info.curr_value;
                build_binary_from_buildables(
                    &Number::IntConst(0),
                    second_exp,
                    program,
                    my_ir_generator_info,
                    BinaryOp::NotEq,
                )?;
                if let Some((tmp2, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    my_ir_generator_info.tmp_constants = Some((tmp1, tmp2));
                }
                let bool2 = my_ir_generator_info.curr_value;

                build_binary_from_values(
                    bool1,
                    bool2,
                    program,
                    my_ir_generator_info,
                    BinaryOp::And,
                )
            }
        }
    }
}
impl Buildable for LOrExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self{
            LOrExp::LAndExp(exp) => exp.build(program, my_ir_generator_info),
            LOrExp::BinaryLOrExp(first_exp, second_exp) => {
                build_binary_from_buildables(
                    &**first_exp,
                    &Number::IntConst(0),
                    program,
                    my_ir_generator_info,
                    BinaryOp::NotEq,
                )?;
                let mut tmp1 = 0;
                if let Some((tmp, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    tmp1 = tmp;
                }
                let bool1 = my_ir_generator_info.curr_value;

                build_binary_from_buildables(
                    &Number::IntConst(0),
                    second_exp,
                    program,
                    my_ir_generator_info,
                    BinaryOp::NotEq,
                )?;
                if let Some((tmp2, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    my_ir_generator_info.tmp_constants = Some((tmp1, tmp2));
                }
                let bool2 = my_ir_generator_info.curr_value;

                build_binary_from_values(
                    bool1,
                    bool2,
                    program,
                    my_ir_generator_info,
                    BinaryOp::Or,
                )
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
    let mut tmp1=0;
    first_exp.build(program, my_ir_generator_info)?;
    let first_value = my_ir_generator_info
        .curr_value;
    if let Some((tmp, _)) = my_ir_generator_info.tmp_constants {
        // Calculating constant expression
        tmp1 = tmp;
    }
    second_exp.build(program, my_ir_generator_info)?;
    let second_value = my_ir_generator_info
        .curr_value;
    if let Some((tmp2, _)) = my_ir_generator_info.tmp_constants {
        // Calculating constant expression
        my_ir_generator_info.tmp_constants = Some((tmp1, tmp2));
    }
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
    first_value: Option<koopa::ir::Value>,
    second_value: Option<koopa::ir::Value>,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<(), String> {
    if let Some((tmp1, tmp2)) = my_ir_generator_info.tmp_constants {
        // Calculating constant expression
        match binary_op {
            BinaryOp::NotEq => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 != tmp2) as i32, 233333))
            }
            BinaryOp::Eq => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 == tmp2) as i32, 233333))
            }
            BinaryOp::Gt => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 > tmp2) as i32, 233333))
            }
            BinaryOp::Lt => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 < tmp2) as i32, 233333))
            }
            BinaryOp::Ge => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 >= tmp2) as i32, 233333))
            }
            BinaryOp::Le => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 <= tmp2) as i32, 233333))
            }
            BinaryOp::Add => {
                my_ir_generator_info.tmp_constants = Some((tmp1 + tmp2, 233333))
            }
            BinaryOp::Sub => {
                my_ir_generator_info.tmp_constants = Some((tmp1 - tmp2, 233333))
            }
            BinaryOp::Mul => {
                my_ir_generator_info.tmp_constants = Some((tmp1 * tmp2, 233333))
            }
            BinaryOp::Div => {
                my_ir_generator_info.tmp_constants = Some((tmp1 / tmp2, 233333))
            }
            BinaryOp::Mod => {
                my_ir_generator_info.tmp_constants = Some((tmp1 % tmp2, 233333))
            }
            BinaryOp::And => {
                my_ir_generator_info.tmp_constants = Some((tmp1 & tmp2, 233333))
            }
            BinaryOp::Or => {
                my_ir_generator_info.tmp_constants = Some((tmp1 | tmp2, 233333))
            }
            BinaryOp::Xor => {
                my_ir_generator_info.tmp_constants = Some((tmp1 ^ tmp2, 233333))
            }
            BinaryOp::Shl => {
                my_ir_generator_info.tmp_constants = Some((tmp1 << tmp2, 233333))
            }
            BinaryOp::Shr => todo!(), // SysY has no left shift or right shift.
            BinaryOp::Sar => {
                my_ir_generator_info.tmp_constants = Some((tmp1 >> tmp2, 233333))
            }
        }
        return Ok(());
    }
    let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
    let new_value =
        curr_func_data
            .dfg_mut()
            .new_value()
            .binary(binary_op, first_value.unwrap(), second_value.unwrap());
    curr_func_data
        .layout_mut()
        .bb_mut(my_ir_generator_info.curr_block.unwrap())
        .insts_mut()
        .extend([new_value]);
    my_ir_generator_info.curr_value = Some(new_value);
    Ok(())
}
