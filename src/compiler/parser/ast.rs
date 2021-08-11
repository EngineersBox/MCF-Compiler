use crate::compiler::parser::varname::VarName;
use crate::compiler::parser::entity::EntityLit;
use crate::compiler::parser::vec_pos::VecPosLit;

pub type ArrayLiteral<'a> = Vec<Field<'a>>;

#[derive(Debug,Clone,PartialEq)]
pub enum Statement<'a> {
    Semicolon,
    Newline,
    Assignment(Assignment<'a>),
    FuncCall(PrefixExp<'a>),
    ForIter(ForIter<'a>),
    FuncDef(FunctionDef<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp<'a> {
    VecPos(VecPosLit<'a>),
    Entity(EntityLit<'a>),
    Array(ArrayLiteral<'a>),
    FuncCall(FunctionCall<'a>),
    PrefixExp(Box<PrefixExp<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForIter<'a> {
    pub vars: Vec<VarName<'a>>,
    pub exps: Vec<Exp<'a>>,
    pub do_blk: Block<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDef<'a> {
    pub name: VarName<'a>,
    pub body: FunctionBody<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionBody<'a> {
    pub params: Params<'a>,
    pub body: Block<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub stmts: Vec<Statement<'a>>,
    pub ret_stmt: Option<Vec<Exp<'a>>>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Params<'a> {
    pub names: Vec<VarName<'a>>,
    pub variadic: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExp<'a> {
    pub prefix: ExpOrVarName<'a>,
    pub suffix_chain: Vec<ExpSuffix<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpOrVarName<'a> {
    Exp(Exp<'a>),
    VarName(VarName<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpSuffix<'a> {
    TableDot(VarName<'a>),
    TableIdx(Exp<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment<'a> {
    pub vars: Vec<PrefixExp<'a>>,
    pub vals: Vec<Exp<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Args<'a> {
    Array(ArrayLiteral<'a>),
    ExpList(Vec<Exp<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall<'a> {
    pub method: Option<VarName<'a>>,
    pub args: Args<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Field<'a> {
    NameAssign(VarName<'a>, Exp<'a>), // Assigning by varname, e.g {foo = 10}
    ExpAssign(Exp<'a>, Exp<'a>), // Assigning by expr, e.g {["foo" .. "bar"] = 10}
    PosAssign(Exp<'a>), // Assigning by position, e.g {"foo", "bar"} assigns in positions 1 and 2
}