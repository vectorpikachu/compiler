use std::io::Write;

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


impl FuncDef {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, var_count: &mut i32) {
        write!(buf, "fun @{}(): ", self.ident).unwrap();
        self.func_type.generate_koopa_ir(buf);
        writeln!(buf, "%entry:").unwrap();
        let mut first_num = 0;
        // 当前块要计算出最里面的数字.
        self.block.generate_koopa_ir(buf, var_count, &mut first_num);
        // writeln!(buf, "  ret 0").unwrap();
        writeln!(buf, "}}").unwrap();
    }
}

#[derive(Debug)]
pub enum FuncType {
    Int,
}

impl FuncType {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        writeln!(buf, "i32 {{").unwrap();
    }
}

#[derive(Debug)]
pub struct Block {
    pub stmt : Stmt,
}
impl Block {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, var_count: &mut i32, 
        first_num: &mut i32) {
        self.stmt.generate_koopa_ir(buf, var_count, first_num);
    }
}

#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
}
impl Stmt {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, var_count: &mut i32,
        first_num: &mut i32) {
        self.exp.generate_koopa_ir(buf, var_count, first_num);
        if *var_count == 0 {
            // 直接树数字， 没有用过变量.
            writeln!(buf, "  ret {}", *first_num).unwrap();
        } else {
            writeln!(buf, "  ret %{}", *var_count - 1).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct Exp {
    pub unary_exp : UnaryExp,
}
impl Exp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, var_count: &mut i32,
        first_num: &mut i32) {
        self.unary_exp.generate_koopa_ir(buf, var_count, first_num);
    }
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
impl UnaryExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, var_count: &mut i32,
        first_num: &mut i32) {
        match self {
            UnaryExp::Unary(op, exp) => {
                exp.generate_koopa_ir(buf, var_count, first_num); // 先计算里层的表达式
                match op {
                    UnaryOp::Add => (), // + 不做任何事
                    UnaryOp::Sub => {
                        if *var_count == 0 {
                            // 直接数字， 没有用过变量.
                            writeln!(buf, "  %{} = sub 0, {}", *var_count, *first_num).unwrap();
                        } else {
                            writeln!(buf, "  %{} = sub 0, %{}", *var_count, *var_count - 1).unwrap();
                        }
                        *var_count = *var_count + 1;
                    }
                    UnaryOp::Rev => {
                        if *var_count == 0 {
                            // 直接数字， 没有用过变量.
                            writeln!(buf, "  %{} = eq {}, 0", *var_count, *first_num).unwrap();
                        } else {
                            writeln!(buf, "  %{} = eq %{}, 0", *var_count, *var_count - 1).unwrap();
                        }
                        *var_count = *var_count + 1;
                    }
                }
            }
            UnaryExp::Primary(exp) => {
                exp.generate_koopa_ir(buf, var_count, first_num);
            }
        }
    }
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<UnaryExp>),
    Number(i32),
}
impl PrimaryExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, var_count: &mut i32,
        first_num: &mut i32) {
        match self {
            PrimaryExp::Exp(unary_exp) => {
                unary_exp.generate_koopa_ir(buf, var_count, first_num);
            }
            PrimaryExp::Number(num) => {
                *first_num = *num;
            }
        }
    }
}


#[derive(Debug)]
pub struct Number(pub i32);


#[derive(Debug)]
pub struct IntConst(pub i32);
