

#[derive(Debug)]
pub struct CompUnit {
    pub func_def : FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}


#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
}


#[derive(Debug)]
pub struct Block {
    pub stmt : Stmt,
}


#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
}


#[derive(Debug)]
pub struct Exp {
    pub l_or_exp : LOrExp,
}


#[derive(Debug)]
pub enum UnaryOp {
    Add,
    Sub,
    Rev, // 取反！
}


#[derive(Debug)]
pub enum UnaryExp {
    UnaryExp(UnaryOp, Box<UnaryExp>),
    PrimaryExp(Box<PrimaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    Number(Number),
}

#[derive(Debug)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum MulExp {
    UnaryExp(UnaryExp),
    MulExp(Box<MulExp>, MulOp, UnaryExp),
}

#[derive(Debug)]
pub enum AddOp {
    Add,
    Sub,
}


#[derive(Debug)]
pub enum AddExp {
    MulExp(MulExp),
    AddExp(Box<AddExp>, AddOp, MulExp),
}

#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}


#[derive(Debug)]
pub struct IntConst(pub i32);

#[derive(Debug)]
pub enum RelOp {
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug)]
pub enum EqOp {
    Eq,
    Ne,
}

#[derive(Debug)]
pub enum LAndOp {
    And,
}

#[derive(Debug)]
pub enum LOrOp {
    Or,
}

#[derive(Debug)]
pub enum RelExp {
    AddExp(AddExp),
    RelExp(Box<RelExp>, RelOp, AddExp),
}

#[derive(Debug)]
pub enum EqExp {
    RelExp(RelExp),
    EqExp(Box<EqExp>, EqOp, RelExp),
}

#[derive(Debug)]
pub enum LAndExp {
    EqExp(EqExp),
    LAndExp(Box<LAndExp>, LAndOp, EqExp),
}

#[derive(Debug)]
pub enum LOrExp {
    LAndExp(LAndExp),
    LOrExp(Box<LOrExp>, LOrOp, LAndExp),
}