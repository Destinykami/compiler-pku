//! Definition Statement of the Abstract Syntax Tree (AST). 

use crate::ast::exp::*;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub return_type: BType,
    pub func_id: String,
    pub block: Block,
}

#[derive(Debug)]
pub struct BType {
    pub type_name: String,
}
#[derive(Debug)]
pub enum Decl{
    ConstDecl(ConstDecl), 
}
#[derive(Debug)]
pub enum ConstDecl{
    ConstDecl(BType,Vec<ConstDef>),
}
#[derive(Debug)]
pub enum ConstDef {
    Default(IDENT, ConstInitVal),
}
#[derive(Debug)]
pub struct IDENT {
    pub content: String,
}

#[derive(Debug)]
pub enum ConstInitVal {
    ConstExp(ConstExp),
}
#[derive(Debug)]
pub enum ConstExp {
    Exp(Exp),
}
#[derive(Debug)]
pub enum LVal {
    IDENT(IDENT),
}
#[derive(Debug)]
/// 代码块
pub enum Block {
    Block(Vec<BlockItem>),
}
#[derive(Debug)]
pub enum BlockItem{
    Decl(Decl),
    Stmt(Stmt),
}
/// Stmt内容
#[derive(Debug)]
pub enum Stmt {
    ReturnStmt(Exp),
    //AssignStmt(LVal,Exp),
}

#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}