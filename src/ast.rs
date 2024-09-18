use std::io::Write;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def : FuncDef,
}

impl CompUnit {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        // do nothing
        self.func_def.generate_koopa_ir(buf);
    }
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}


impl FuncDef {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        write!(buf, "fun @{}(): ", self.ident).unwrap();
        self.func_type.generate_koopa_ir(buf);
        writeln!(buf, "%entry:").unwrap();
        writeln!(buf, "    ret 0").unwrap();
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

#[derive(Debug)]
pub struct Stmt {
    pub num : i32,
}


#[derive(Debug)]
pub struct IntConst(pub i32);
