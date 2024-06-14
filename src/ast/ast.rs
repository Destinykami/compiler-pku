//! Definition of the Abstract Syntax Tree (AST). 
//! 
//!
//! Currently, AST is defined as follows:
//!
//! CompUnit  ::= FuncDef;
//!
//! FuncDef   ::= FuncType Id "(" ")" Block;
//! FuncType  ::= "int";
//!
//! Block     ::= "{" Stmt "}";
//! 文法变更
//! Stmt        ::= "return" Exp ";";
//! Exp         ::= AddExp;
//! PrimaryExp  ::= "(" Exp ")" | Number;
//! Number      ::= INT_CONST;
//! UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
//! UnaryOp     ::= "+" | "-" | "!";
//! MulExp      ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
//! AddExp      ::= MulExp | AddExp ("+" | "-") MulExp;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub return_type: Type,
    pub func_id: String,
    pub block: Block,
}

#[derive(Debug)]
pub struct Type {
    pub type_name: String,
}

#[derive(Debug)]
/// 代码块
pub struct Block {
    pub stmt: Stmt,
}
/// Stmt内容

#[derive(Debug)]
pub enum Stmt {
    ReturnStmt(Exp),
}

#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}

#[derive(Debug)]
pub enum Exp{
    AddExp(AddExp)
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