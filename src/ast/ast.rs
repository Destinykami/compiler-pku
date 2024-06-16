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
//! Decl          ::= ConstDecl;
//! ConstDecl     ::= "const" BType ConstDef {"," ConstDef} ";";
//! BType         ::= "int";
//! ConstDef      ::= IDENT "=" ConstInitVal;
//! ConstInitVal  ::= ConstExp;
//! 
//! Block         ::= "{" {BlockItem} "}";
//! BlockItem     ::= Decl | Stmt;
//! LVal          ::= IDENT;
//! ConstExp      ::= Exp;

//! 文法变更
//! Stmt        ::= "return" Exp ";";
//!                 | LVal "=" Exp ";"
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
