use std::io::{stderr, Write};
use crate::ast::*;

pub struct GenerateIRParams {
    pub var_count: i32,
    pub first_num: i32,
}

impl CompUnit {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        // do nothing
        // 计算整个程序的变量计数
        let mut params = GenerateIRParams {
            var_count: 0,
            first_num: 0,
        };
        self.func_def.generate_koopa_ir(buf, &mut params);
    }
}

impl FuncDef {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        if self.ident != "main" {
            stderr().write_all(b"Error: only support main function\n").unwrap();
            return;
        }
        write!(buf, "fun @{}(): ", self.ident).unwrap();
        self.func_type.generate_koopa_ir(buf);
        params.first_num = 0;
        // 当前块要计算出最里面的数字.
        self.block.generate_koopa_ir(buf, params);
        writeln!(buf, "}}").unwrap();
    }
}

impl FuncType {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        match self {
            FuncType::Void => {
                writeln!(buf, "void {{").unwrap();
            }
            FuncType::Int => {
                writeln!(buf, "i32 {{").unwrap();
            }
        }
    }
}

impl Block {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        writeln!(buf, "%entry:").unwrap();
        self.stmt.generate_koopa_ir(buf, params);
    }
}

impl Stmt {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        self.exp.generate_koopa_ir(buf, params);
        if params.var_count == 0 {
            // 直接是数字， 没有用过变量.
            writeln!(buf, "  ret {}", params.first_num).unwrap();
        } else {
            writeln!(buf, "  ret %{}", params.var_count - 1).unwrap();
        }
    }
}

impl Exp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        self.unary_exp.generate_koopa_ir(buf, params);
    }
}

impl UnaryExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        match self {
            UnaryExp::Unary(op, exp) => {
                exp.generate_koopa_ir(buf, params); // 先计算里层的表达式
                match op {
                    UnaryOp::Add => (), // + 不做任何事
                    UnaryOp::Sub => {
                        if params.var_count == 0 {
                            // 直接数字， 没有用过变量.
                            writeln!(buf, "  %{} = sub 0, {}", params.var_count, params.first_num).unwrap();
                        } else {
                            writeln!(buf, "  %{} = sub 0, %{}", params.var_count, params.var_count - 1).unwrap();
                        }
                        params.var_count = params.var_count + 1;
                    }
                    UnaryOp::Rev => {
                        if params.var_count == 0 {
                            // 直接数字， 没有用过变量.
                            writeln!(buf, "  %{} = eq {}, 0", params.var_count, params.first_num).unwrap();
                        } else {
                            writeln!(buf, "  %{} = eq %{}, 0", params.var_count, params.var_count - 1).unwrap();
                        }
                        params.var_count = params.var_count + 1;
                    }
                }
            }
            UnaryExp::Primary(exp) => {
                exp.generate_koopa_ir(buf, params);
            }
        }
    }
}


impl PrimaryExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        match self {
            PrimaryExp::Exp(unary_exp) => {
                unary_exp.generate_koopa_ir(buf, params);
            }
            PrimaryExp::Number(num) => {
                params.first_num = *num;
            }
        }
    }
}