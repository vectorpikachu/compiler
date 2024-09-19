// 根据内存形式 Koopa IR 生成汇编
use koopa::ir::ValueKind;
use std::io::Write;
use koopa::ir::*;

pub struct GenerateAsmParams {
    pub register_count: i32, // 使用的临时存放变量的寄存器的数量
    pub register_data : Vec<i32>, // 临时存放变量的寄存器的数据
}

pub fn register_count_to_name(register_count: i32) -> String {
    if register_count >= 1 && register_count <= 7 {
        return format!("t{}", register_count - 1);
    } else {
        return format!("a{}", register_count - 8);
    }
}

pub trait GenerateAsm {
    fn generate_asm(&self, buf: &mut Vec<u8>, params: &mut GenerateAsmParams /* 其他必要的参数 */);
}

impl GenerateAsm for koopa::ir::Program {
    fn generate_asm(&self, buf: &mut Vec<u8>, params: &mut GenerateAsmParams) {
        writeln!(buf, "  .text").unwrap();
        // writeln!(buf, "  .globl main").unwrap();
        for &func in self.func_layout() {
            self.func(func).generate_asm(buf, params);
        }
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate_asm(&self, buf: &mut Vec<u8>, params: &mut GenerateAsmParams) {
        // 首先提供函数入口
        writeln!(buf, "  .globl {}", self.name().replace("@", "")).unwrap();
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
                        let ret_value_data = ret_value.value();
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
                    ValueKind::Binary(bin) => {
                        // 获取二元操作数
                        let bin_op = bin.op();
                        let bin_lhs = bin.lhs();
                        let bin_rhs = bin.rhs();
                        let lhs_data = self.dfg().value(bin_lhs);
                        let rhs_data = self.dfg().value(bin_rhs);
                        let mut lhs_reg = String::new();
                        let mut rhs_reg = String::new();
                        match lhs_data.kind() {
                            ValueKind::Integer(int_num) => {
                                if int_num.value() != 0 {
                                    params.register_count += 1;
                                    params.register_data.push(int_num.value()); 
                                    lhs_reg = register_count_to_name(params.register_count);
                                    writeln!(buf, "  li {}, {}", lhs_reg.clone(), int_num.value()).unwrap();
                                } else {
                                    lhs_reg = "x0".to_string();
                                }
                            }
                            _ => {}
                        }
                        match rhs_data.kind() {
                            ValueKind::Integer(int_num) => {
                                if int_num.value() != 0 {
                                    params.register_count += 1;
                                    params.register_data.push(int_num.value()); 
                                    rhs_reg = register_count_to_name(params.register_count);
                                    writeln!(buf, "  li {}, {}", rhs_reg, int_num.value()).unwrap();
                                } else {
                                    rhs_reg = "x0".to_string();
                                }
                            }
                            _ => {}
                        }
                        match bin_op {
                            BinaryOp::Add => {
                                params.register_count += 1;
                                writeln!(buf, "  add {}, {}, {}", register_count_to_name(params.register_count), lhs_reg, rhs_reg).unwrap();
                            }
                            BinaryOp::Sub => {
                                params.register_count += 1;
                                rhs_reg = register_count_to_name(params.register_count - 1);
                                writeln!(buf, "  sub {}, {}, {}", register_count_to_name(params.register_count), lhs_reg, rhs_reg).unwrap();
                            }
                            _ => {}
                        }

                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
