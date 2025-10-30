use std::collections::HashMap;

pub type Id = String;
pub type BasicBlockId = usize;
pub type Field = String;
pub type StructName = String;
pub type FunctionName = String;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Enum(String),
    Ptr(Box<Type>),
    Struct(StructName),
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}

pub enum UnOp {
    Neg,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum RValue {
    UseInt(i64),
    Copy(Id),
    UnaryOp(UnOp, Id),
    BinaryOp(BinOp, Id, Id),
    AddrOf(Id),
    Deref(Id),
    ProjectField(Id, Field),
    ProjectIndex(Id, Id),
    UseFn(FunctionName),
}

pub enum Statement {
    Assign(Id, RValue),
    DerefAssign(Id, Id),
    FieldAssign(Id, Field, Id),
    IndexAssign(Id, Id, Id),
}

pub enum Terminator {
    Goto(BasicBlockId),
    SwitchInt(Id, Vec<(i64, BasicBlockId)>, BasicBlockId),
    Return,
    Call(Id, FunctionName, Vec<Id>, BasicBlockId),
    CallIndirect(Id, Id, Vec<Id>, BasicBlockId),
}

pub struct BasicBlock {
    pub statements: Vec<Statement>,
    pub terminator: Terminator,
}

pub struct Function {
    pub name: FunctionName,
    pub blocks: HashMap<BasicBlockId, BasicBlock>,
    pub locals: HashMap<Id, Type>,
}

pub struct Program {
    pub structs: HashMap<StructName, HashMap<Field, Type>>,
    pub functions: HashMap<FunctionName, Function>,
}
