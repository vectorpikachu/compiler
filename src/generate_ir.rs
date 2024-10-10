use std::{collections::HashMap, io::{stderr, Write}};
use crate::ast::*;

pub struct GenerateIRParams {
    pub var_count: i32,
    // pub first_num: i32,
    pub sym_tab: HashMap<String, i32>,
}

pub enum ExpResult {
    RegCount(i32),
    IntResult(i32),
}

impl CompUnit {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        // do nothing
        // 计算整个程序的变量计数
        let mut params = GenerateIRParams {
            var_count: 0,
            // first_num: 0,
            sym_tab: HashMap::new(),
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
        for block_item in &self.block_items {
            block_item.generate_koopa_ir(buf, params);
        }
    }
}

impl BlockItem {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        match self {
            BlockItem::Stmt(stmt) => {
                stmt.generate_koopa_ir(buf, params);
            }
            BlockItem::Decl(decl) => {
                decl.calc_const(params);
            }
        }
    }
}

impl Decl {
    pub fn calc_const(&self, params: &mut GenerateIRParams) {
        self.const_decl.calc_const(params);
    }
}

impl ConstDecl {
    pub fn calc_const(&self, params: &mut GenerateIRParams) {
        // 直接存到符号表里
        for const_def in &self.const_defs {
            const_def.calc_const(params);
        }
    }
}

impl ConstDef {
    pub fn calc_const(&self, params: &mut GenerateIRParams) {
        // 直接存到符号表里
        let init_val = self.const_init_val.calc_const(params);
        params.sym_tab.insert(self.ident.clone(), init_val);
    }
}

impl ConstInitVal {
    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        return self.const_exp.calc_const(params);
    }
}

impl ConstExp {
    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        return self.exp.calc_const(params);
    }
}

impl Stmt {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        let exp_res = self.exp.generate_koopa_ir(buf, params);
        match exp_res {
            ExpResult::RegCount(reg_count) => {
                writeln!(buf, "  ret %{}", reg_count-1).unwrap();
            }
            ExpResult::IntResult(int_res) => {
                writeln!(buf, "  ret {}", int_res).unwrap();
            }
        }
    }
}

impl Exp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        return self.l_or_exp.generate_koopa_ir(buf, params);
    }
    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        return self.l_or_exp.calc_const(params);
    }
}

impl UnaryExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult{
        match self {
            UnaryExp::UnaryExp(unary_op, unary_exp) => {
                let unary_exp_res = unary_exp.generate_koopa_ir(buf, params); // 先计算里层的表达式
                
                let mut unary_exp_buf: Vec<u8> = Vec::new();
                match unary_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        write!(unary_exp_buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        write!(unary_exp_buf, "{}", int_res).unwrap();
                    }
                }
                
                let unary_exp_str = String::from_utf8(unary_exp_buf).unwrap();

                match unary_op {
                    UnaryOp::Add => {
                        return unary_exp_res;
                    } 
                    UnaryOp::Sub => {
                        writeln!(buf, "  %{} = sub 0, {}", params.var_count, 
                            unary_exp_str).unwrap();
                    }
                    UnaryOp::Rev => {
                        writeln!(buf, "  %{} = eq {}, 0", params.var_count, 
                            unary_exp_str).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;
                let exp_res = ExpResult::RegCount(params.var_count);
                return exp_res;
            }
            UnaryExp::PrimaryExp(primary_exp) => {
                return primary_exp.generate_koopa_ir(buf, params);
            }
        }
    }
}


impl PrimaryExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            PrimaryExp::Exp(exp) => {
                return exp.generate_koopa_ir(buf, params);
            }
            PrimaryExp::Number(num) => {
                match num {
                    Number::IntConst(num) => {
                        return ExpResult::IntResult(*num);
                    }
                }
            }
            PrimaryExp::LVal(l_val) => {
                // do nothing
                return ExpResult::IntResult(0);
            }
        }
    }
}

impl AddExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            AddExp::MulExp(mul_exp) => {
                return mul_exp.generate_koopa_ir(buf, params);
            }
            AddExp::AddExp(add_exp, add_op, mul_exp) => {
                let mul_exp_res = mul_exp.generate_koopa_ir(buf, params);
                let add_exp_res = add_exp.generate_koopa_ir(buf, params);
                match add_op {
                    AddOp::Add => {
                        write!(buf, "  %{} = add ", params.var_count).unwrap();
                    }
                    AddOp::Sub => {
                        write!(buf, "  %{} = sub ", params.var_count).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match add_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        write!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        write!(buf, "{}", int_res).unwrap();
                    }
                }
                write!(buf, ", ").unwrap();
                match mul_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "{}", int_res).unwrap();
                    }
                }
                let res = ExpResult::RegCount(params.var_count);
                return res;
            }
        }
    }
}

impl MulExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            MulExp::UnaryExp(unary_exp) => {
                return unary_exp.generate_koopa_ir(buf, params);
            }
            MulExp::MulExp(mul_exp, mul_op, unary_exp) => {
                let mul_exp_res = mul_exp.generate_koopa_ir(buf, params);
                let unary_exp_res = unary_exp.generate_koopa_ir(buf, params);

                match mul_op {
                    MulOp::Mul => {
                        write!(buf, "  %{} = mul ", params.var_count).unwrap();
                    }
                    MulOp::Div => {
                        write!(buf, "  %{} = div ", params.var_count).unwrap();
                    }
                    MulOp::Mod => {
                        write!(buf, "  %{} = mod ", params.var_count).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match mul_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        write!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        write!(buf, "{}", int_res).unwrap();
                    }
                }
                write!(buf, ", ").unwrap();
                match unary_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "{}", int_res).unwrap();
                    }
                }
                let res = ExpResult::RegCount(params.var_count);
                return res;
            }
        }
    }
}

impl RelExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            RelExp::AddExp(add_exp) => {
                return add_exp.generate_koopa_ir(buf, params);
            }
            RelExp::RelExp(rel_exp, rel_op, add_exp) => {
                let rel_exp_res = rel_exp.generate_koopa_ir(buf, params);
                let add_exp_res = add_exp.generate_koopa_ir(buf, params);

                match rel_op {
                    RelOp::Lt => {
                        write!(buf, "  %{} = lt ", params.var_count).unwrap();
                    }
                    RelOp::Gt => {
                        write!(buf, "  %{} = gt ", params.var_count).unwrap();
                    }
                    RelOp::Le => {
                        write!(buf, "  %{} = le ", params.var_count).unwrap();
                    }
                    RelOp::Ge => {
                        write!(buf, "  %{} = ge ", params.var_count).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match rel_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        write!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        write!(buf, "{}", int_res).unwrap();
                    }
                }
                write!(buf, ", ").unwrap();
                match add_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "{}", int_res).unwrap();
                    }
                }
                let res = ExpResult::RegCount(params.var_count);
                return res;
            }
        }
    }
}

impl EqExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            EqExp::RelExp(rel_exp) => {
                return rel_exp.generate_koopa_ir(buf, params);
            }
            EqExp::EqExp(eq_exp, eq_op, rel_exp) => {
                let eq_exp_res = eq_exp.generate_koopa_ir(buf, params);
                let rel_exp_res = rel_exp.generate_koopa_ir(buf, params);

                match eq_op {
                    EqOp::Eq => {
                        write!(buf, "  %{} = eq ", params.var_count).unwrap();
                    }
                    EqOp::Ne => {
                        write!(buf, "  %{} = ne ", params.var_count).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match eq_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        write!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        write!(buf, "{}", int_res).unwrap();
                    }
                }
                write!(buf, ", ").unwrap();
                match rel_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "%{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "{}", int_res).unwrap();
                    }
                }
                let res = ExpResult::RegCount(params.var_count);
                return res;
            }
        }
    }
}

impl LAndExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            LAndExp::EqExp(eq_exp) => {
                return eq_exp.generate_koopa_ir(buf, params);
            }
            LAndExp::LAndExp(l_and_exp, l_and_op, eq_exp) => {
                let l_and_exp_res = l_and_exp.generate_koopa_ir(buf, params);
                let eq_exp_res = eq_exp.generate_koopa_ir(buf, params);
                /* 逻辑与应该是 and (ne lhs 0) (ne rhs 0)  */

                match l_and_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "  %{} = ne %{}, 0", params.var_count, 
                            reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "  %{} = ne {}, 0", params.var_count, 
                            int_res).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match eq_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "  %{} = ne %{}, 0", params.var_count, 
                            reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "  %{} = ne {}, 0", params.var_count, 
                            int_res).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match l_and_op {
                    LAndOp::And => {
                        writeln!(buf, "  %{} = and %{}, %{}", params.var_count,
                            params.var_count-2,
                            params.var_count-1).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;
                
                let res = ExpResult::RegCount(params.var_count);
                return res;
            }
        }
    }
}

impl LOrExp {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        match self {
            LOrExp::LAndExp(l_and_exp) => {
                return l_and_exp.generate_koopa_ir(buf, params);
            }
            LOrExp::LOrExp(l_or_exp, l_or_op, l_and_exp) => {
                let l_or_exp_res = l_or_exp.generate_koopa_ir(buf, params);
                let l_and_exp_res = l_and_exp.generate_koopa_ir(buf, params);

                match l_or_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "  %{} = ne %{}, 0", params.var_count, 
                            reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "  %{} = ne {}, 0", params.var_count, 
                            int_res).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match l_and_exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "  %{} = ne %{}, 0", params.var_count, 
                            reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "  %{} = ne {}, 0", params.var_count, 
                            int_res).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                match l_or_op {
                    LOrOp::Or => {
                        writeln!(buf, "  %{} = or %{}, %{}", params.var_count,
                            params.var_count-2,
                            params.var_count-1).unwrap();
                    }
                }
                params.var_count = params.var_count + 1;

                let res = ExpResult::RegCount(params.var_count);
                return res;
            }
        }
    }

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        
    }
}