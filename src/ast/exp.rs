//! Definition Exp of the Abstract Syntax Tree (AST). 
//! Currently, AST is defined as follows:
//!
//! CompUnit  ::= FuncDef;
//! FuncDef   ::= FuncType Id "(" ")" Block;
//! FuncType  ::= "int";
//! Decl          ::= ConstDecl;
//! ConstDecl     ::= "const" BType ConstDef {"," ConstDef} ";";
//! BType         ::= "int";
//! ConstDef      ::= IDENT "=" ConstInitVal;
//! ConstInitVal  ::= ConstExp;
//! Block         ::= "{" {BlockItem} "}";
//! BlockItem     ::= Decl | Stmt;
//! LVal          ::= IDENT;
//! ConstExp      ::= Exp;
//! Stmt        ::= "return" Exp ";";
//!                 | LVal "=" Exp ";"
//! This file include:
//! Exp         ::= LOrExp;
//! PrimaryExp  ::= "(" Exp ")" | Number | LVal;
//! Number      ::= INT_CONST;
//! UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
//! UnaryOp     ::= "+" | "-" | "!";
//! MulExp      ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
//! AddExp      ::= MulExp | AddExp ("+" | "-") MulExp;
//! RelExp      ::= AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp;
//! EqExp       ::= RelExp | EqExp ("==" | "!=") RelExp;
//! LAndExp     ::= EqExp | LAndExp "&&" EqExp;
//! LOrExp      ::= LAndExp | LOrExp "||" LAndExp;

use super::statements::*;

#[derive(Debug)]
pub enum Exp{
    LOrExp(LOrExp)
}
#[derive(Debug)]
pub enum UnaryExp{
    //一元表达式
    PrimaryExp(PrimaryExp),
    PlusUnaryExp(Box<UnaryExp>), 
    MinusUnaryExp(Box<UnaryExp>), 
    NotUnaryExp(Box<UnaryExp>), 
}
#[derive(Debug)]
pub enum PrimaryExp{
    BracedExp(Box<Exp>),
    Number(Number),
    LVal(LVal),
}
#[derive(Debug)]
pub enum MulExp {
    UnaryExp(UnaryExp),
    BinaryMulExp(Box<MulExp>, UnaryExp), 
    BinaryDivExp(Box<MulExp>, UnaryExp), 
    BinaryModExp(Box<MulExp>, UnaryExp), 

}
#[derive(Debug)]
pub enum AddExp {
    MulExp(MulExp),
    BinaryAddExp(Box<AddExp>, MulExp), 
    BinarySubExp(Box<AddExp>, MulExp), 
}
#[derive(Debug)]
pub enum LOrExp {
    LAndExp(LAndExp),
    BinaryLOrExp(Box<LOrExp>,LAndExp),
}
#[derive(Debug)]
pub enum  LAndExp {
    EqExp(EqExp),
    BinaryLAndExp(Box<LAndExp>,EqExp),
}
#[derive(Debug)]
pub enum EqExp {
    RelExp(RelExp),
    BinaryEqExp(Box<EqExp>,RelExp),
    BinaryNotEqExp(Box<EqExp>,RelExp),
}
#[derive(Debug)]
pub enum RelExp {
    AddExp(AddExp),
    BinaryLtRelExp(Box<RelExp>,AddExp),//小于
    BinaryGtRelExp(Box<RelExp>,AddExp),//大于
    BinaryLeRelExp(Box<RelExp>,AddExp),//小于等于
    BinaryGeRelExp(Box<RelExp>,AddExp),//大于等于
}
