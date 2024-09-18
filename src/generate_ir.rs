use std::io::Write;
use crate::ast::*;

impl CompUnit {
    pub fn generate_koopa_ir(&self, buf: &mut Vec<u8>) {
        // do nothing
        // 计算整个程序的变量计数
        let mut var_count = 0;
        self.func_def.generate_koopa_ir(buf, &mut var_count);
    }
}