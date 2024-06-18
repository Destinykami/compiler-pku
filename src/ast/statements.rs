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
    VarDecl(VarDecl),
}
#[derive(Debug)]
pub enum ConstDecl{
    ConstDecl(BType,Vec<ConstDef>),
}
#[derive(Debug)]
pub enum ConstDef {
    ConstDef(IDENT, ConstInitVal),
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
pub enum VarDecl{
    VarDecl(BType,Vec<VarDef>),
}
#[derive(Debug)]
pub enum VarDef {
    VarDef(IDENT, InitVal),
    IDENT(IDENT),
}
#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
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
    AssignStmt(LVal,Exp),
}

#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}