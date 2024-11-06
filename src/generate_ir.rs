use std::{collections::HashMap, io::{stderr, Write}};
use crate::ast::*;

pub struct GenerateIRParams {
    pub var_count: i32,
    pub func_returned: bool,
    // pub first_num: i32,
    pub sym_tab: HashMap<String, SymVal>, //只有一个过程的话符号表不能同名的
}

pub enum ExpResult {
    RegCount(i32),
    IntResult(i32),
}

pub enum SymVal {
    ConstVal(i32),
    VarName,
}

impl CompUnit {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        // do nothing
        // 计算整个程序的变量计数
        let mut params = GenerateIRParams {
            var_count: 0,
            func_returned: false,
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
        if params.func_returned == false {
            // 没有return语句
            writeln!(buf, "  ret undef").unwrap();
        }
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
            if params.func_returned == true {
                break;
            }
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
               decl.generate_koopa_ir(buf, params);
            }
        }
    }
}

impl Decl {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        match self {
            Decl::ConstDecl(const_decl) => {
                const_decl.calc_const(params);
            }
            Decl::VarDecl(var_decl) => {
                var_decl.generate_koopa_ir(buf, params);
            }
        }
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
        params.sym_tab.insert(self.ident.clone(), SymVal::ConstVal(init_val));
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

impl VarDecl {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        for var_def in &self.var_defs {
            var_def.generate_koopa_ir(buf, params);
        }
    }
}

impl VarDef {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        // 首先使用alloc命令, 接着根据是否有初值来计算.
        match self {
            VarDef::VarDefUninit(var_name) => {
                writeln!(buf, "  @{} = alloc i32", var_name).unwrap();
                // 存入符号表中
                params.sym_tab.insert(var_name.clone(), SymVal::VarName);
            }
            VarDef::VarDefInit(var_name, init_val) => {
                writeln!(buf, "  @{} = alloc i32", var_name).unwrap();
                let val_result = init_val.generate_koopa_ir(buf, params);
                match val_result {
                    ExpResult::RegCount(reg) => {
                        writeln!(buf, "  store %{}, @{}", reg-1, var_name).unwrap();
                    }
                    ExpResult::IntResult(int_val) => {
                        writeln!(buf, "  store {}, @{}", int_val, var_name).unwrap();
                    }
                }
                // 存入符号表中
                params.sym_tab.insert(var_name.clone(), SymVal::VarName);
            }
        }
    }
}

impl InitVal {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        return self.exp.generate_koopa_ir(buf, params);
    } 
}

impl Stmt {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) {
        match self {
            Stmt::Return(exp) => {
                let exp_res = exp.generate_koopa_ir(buf, params);
                match exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "  ret %{}", reg_count-1).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "  ret {}", int_res).unwrap();
                    }
                }
                params.func_returned = true;
            }
            Stmt::Assgn(l_val, exp) => {
                let exp_res = exp.generate_koopa_ir(buf, params);
                match exp_res {
                    ExpResult::RegCount(reg_count) => {
                        writeln!(buf, "  store %{}, @{}", reg_count-1, l_val.ident).unwrap();
                    }
                    ExpResult::IntResult(int_res) => {
                        writeln!(buf, "  store {}, @{}", int_res, l_val.ident).unwrap();
                    }
                }
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

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            UnaryExp::UnaryExp(unary_op, unary_exp) => {
                let unary_exp_res = unary_exp.calc_const(params); // 先计算里层的表达式
                match unary_op {
                    UnaryOp::Add => {
                        return unary_exp_res;
                    } 
                    UnaryOp::Sub => {
                        return -unary_exp_res;
                    }
                    UnaryOp::Rev => {
                        // println!("!{} = {}", unary_exp_res, 1);
                        // stdout().write_all(b"rev is called\n").unwrap();
                        return if unary_exp_res == 0 {1} else {0}
                    }
                }
            }
            UnaryExp::PrimaryExp(primary_exp) => {
                return primary_exp.calc_const(params);
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
                return l_val.generate_koopa_ir(buf, params);
            }
        }
    }

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            PrimaryExp::Exp(exp) => {
                return exp.calc_const(params);
            }
            PrimaryExp::Number(num) => {
                match num {
                    Number::IntConst(num) => {
                        return *num;
                    }
                }
            }
            PrimaryExp::LVal(l_val) => {
                return l_val.calc_const(params);
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

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            AddExp::MulExp(mul_exp) => {
                return mul_exp.calc_const(params);
            }
            AddExp::AddExp(add_exp, add_op, mul_exp) => {
                let mul_exp_res = mul_exp.calc_const(params);
                let add_exp_res = add_exp.calc_const(params);

                match add_op {
                    AddOp::Add => {
                        // println!("{} + {}", add_exp_res, mul_exp_res);
                        // stdout().write_all(b"add is called\n").unwrap();
                        return add_exp_res + mul_exp_res;
                    }
                    AddOp::Sub => {
                        // println!("{} - {}", add_exp_res, mul_exp_res);
                        // stdout().write_all(b"sub is called\n").unwrap();
                        return add_exp_res - mul_exp_res;
                    }
                }
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

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            MulExp::UnaryExp(unary_exp) => {
                return unary_exp.calc_const(params);
            }
            MulExp::MulExp(mul_exp, mul_op, unary_exp) => {
                let mul_exp_res = mul_exp.calc_const(params);
                let unary_exp_res = unary_exp.calc_const(params);

                match mul_op {
                    MulOp::Mul => {
                        return mul_exp_res * unary_exp_res;
                    }
                    MulOp::Div => {
                        return mul_exp_res / unary_exp_res;
                    }
                    MulOp::Mod => {
                        return mul_exp_res % unary_exp_res;
                    }
                }
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

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            RelExp::AddExp(add_exp) => {
                return add_exp.calc_const(params);
            }
            RelExp::RelExp(rel_exp, rel_op, add_exp) => {
                let rel_exp_res = rel_exp.calc_const(params);
                let add_exp_res = add_exp.calc_const(params);

                match rel_op {
                    RelOp::Lt => {
                        return if rel_exp_res < add_exp_res {1} else {0};
                    }
                    RelOp::Gt => {
                        return if rel_exp_res > add_exp_res {1} else {0};
                    }
                    RelOp::Le => {
                        return if rel_exp_res <= add_exp_res {1} else {0};
                    }
                    RelOp::Ge => {
                        return if rel_exp_res >= add_exp_res {1} else {0};
                    }
                }
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

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            EqExp::RelExp(rel_exp) => {
                return rel_exp.calc_const(params);
            }
            EqExp::EqExp(eq_exp, eq_op, rel_exp) => {
                let eq_exp_res = eq_exp.calc_const(params);
                let rel_exp_res = rel_exp.calc_const(params);

                match eq_op {
                    EqOp::Eq => {
                        return if eq_exp_res == rel_exp_res {1} else {0};
                    }
                    EqOp::Ne => {
                        return if eq_exp_res != rel_exp_res {1} else {0};
                    }
                }
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

    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        match self {
            LAndExp::EqExp(eq_exp) => {
                return eq_exp.calc_const(params);
            }
            LAndExp::LAndExp(l_and_exp, l_and_op, eq_exp) => {
                let l_and_exp_res = l_and_exp.calc_const(params);
                let eq_exp_res = eq_exp.calc_const(params);

                match l_and_op {
                    LAndOp::And => {
                        return if l_and_exp_res != 0 && eq_exp_res != 0 {1} else {0};
                    }
                }
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
        match self {
            LOrExp::LAndExp(l_and_exp) => {
                return l_and_exp.calc_const(params);
            }
            LOrExp::LOrExp(l_or_exp, l_or_op, l_and_exp) => {
                let l_or_exp_res = l_or_exp.calc_const(params);
                let l_and_exp_res = l_and_exp.calc_const(params);

                match l_or_op {
                    LOrOp::Or => {
                        return if l_or_exp_res != 0 || l_and_exp_res != 0 {1} else {0};
                    }
                }
            }
        }
    }
}

impl LVal {
    pub fn calc_const(&self, params: &mut GenerateIRParams) -> i32 {
        let var_val = params.sym_tab.get(&self.ident).unwrap();
        match *var_val {
            SymVal::ConstVal(res) => return res,
            SymVal::VarName => {
                stderr().write_all(b"Error: variables occurred in const init val.\n").unwrap();
                return 0;
            }
        }
    }
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>, params: &mut GenerateIRParams) -> ExpResult {
        let var_val = params.sym_tab.get(&self.ident).unwrap();
        match *var_val {
            SymVal::ConstVal(res) => return ExpResult::IntResult(res),
            SymVal::VarName => {
                writeln!(buf, "  %{} = load @{}", params.var_count, self.ident).unwrap();
                params.var_count = params.var_count + 1;
                return ExpResult::RegCount(params.var_count);
            }
        }
    }
}