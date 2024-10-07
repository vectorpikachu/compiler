
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
    pub unary_exp : UnaryExp,
}


#[derive(Debug)]
pub enum UnaryOp {
    Add,
    Sub,
    Rev, // 取反！
}


#[derive(Debug)]
pub enum UnaryExp {
    Unary(UnaryOp, Box<UnaryExp>),
    Primary(Box<PrimaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<UnaryExp>),
    Number(Number),
}

#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}


#[derive(Debug)]
pub struct IntConst(pub i32);
