// 根据内存形式 Koopa IR 生成汇编
use koopa::ir::ValueKind;
use std::io::Write;

pub trait GenerateAsm {
    fn generate_asm(&self, buf: &mut Vec<u8> /* 其他必要的参数 */);
}

impl GenerateAsm for koopa::ir::Program {
    fn generate_asm(&self, buf: &mut Vec<u8>) {
        writeln!(buf, "  .text").unwrap();
        // writeln!(buf, "  .globl main").unwrap();
        for &func in self.func_layout() {
            self.func(func).generate_asm(buf);
        }
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate_asm(&self, buf: &mut Vec<u8>) {
        // 首先提供函数入口
        writeln!(buf, "  .globl main").unwrap();
        writeln!(buf, "{}:", self.name().replace("@", "")).unwrap();
        // ...
        // 访问基本块
        for (&bb, node) in self.layout().bbs() {
            // 访问指令列表
            for &inst in node.insts().keys() {
                let value_data = self.dfg().value(inst);
                // value_data 是 &ValueData 类型
                match value_data.kind() {
                    ValueKind::Integer(int_num) => {
                        // 发现整数数据
                        writeln!(buf, "  li a0, {}", int_num.value()).unwrap();
                    }
                    ValueKind::Return(ret_value) => {
                        // 获取返回值
                        let mut ret_value_data = ret_value.value();
                        match ret_value_data {
                            Some(data) => {
                                match self.dfg().value(data).kind() {
                                    ValueKind::Integer(int_num) => {
                                        writeln!(buf, "  li a0, {}", int_num.value()).unwrap();
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }

                        writeln!(buf, "  ret").unwrap();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
