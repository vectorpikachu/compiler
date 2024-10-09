// 根据内存形式 Koopa IR 生成汇编
use koopa::ir::ValueKind;
use std::io::Write;
use koopa::ir::*;
use std::collections::HashMap;

pub struct GenerateAsmParams {
    pub current_register: i32, // 当前使用的register编号
    pub register_data : HashMap<Value, i32>, // 寄存器中的数据
    pub register_usage : HashMap<i32, bool>, // 寄存器的使用情况
}

pub fn register_idx_to_name(register_idx: i32) -> String {
    if register_idx >= 0 && register_idx <= 6 {
        return format!("t{}", register_idx);
    } else {
        return format!("a{}", register_idx - 7);
    }
}

pub fn available_register(register_usage: &HashMap<i32, bool>) -> i32 {
    let mut i = 0;
    while i < 15 {
        let usage = register_usage.get(&i).unwrap();
        if *usage == false {
            return i
        }
        i = i + 1;
    }
    /* `i32` value */
    return 0
}

pub fn load_operation(int_num: i32, inst: Value, buf: &mut Vec<u8>, params: &mut GenerateAsmParams) {
    let reg = available_register(&params.register_usage);
    let reg_str = register_idx_to_name(reg);
    params.current_register = reg;
    params.register_data.insert(inst, reg);
    params.register_usage.insert(reg, true);
    writeln!(buf, "  li {}, {}", reg_str, int_num).unwrap();
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
        for (&_bb, node) in self.layout().bbs() {
            // 访问指令列表
            for &inst in node.insts().keys() {
                let inst_data = self.dfg().value(inst);
                // value_data 是 &ValueData 类型
                match inst_data.kind() {
                    ValueKind::Integer(int_num) => {
                        // 发现整数数据
                        load_operation(int_num.value(), inst, buf, params);
                    }
                    ValueKind::Return(ret_value) => {
                        // 获取返回值
                        let ret_value_data = ret_value.value().unwrap();
                        let reg = params.register_data.get(&ret_value_data);
                        match reg {
                            Some(reg_idx) => {
                                let reg_str = register_idx_to_name(*reg_idx);
                                writeln!(buf, "  mv a0, {}", reg_str).unwrap();
                            }
                            None => {
                                match self.dfg().value(ret_value_data).kind() {
                                    ValueKind::Integer(int_num) => {
                                        writeln!(buf, "  li a0, {}", int_num.value()).unwrap();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        writeln!(buf, "  ret").unwrap();
                    }
                    ValueKind::Binary(bin) => {
                        // 获取二元操作数
                        let bin_op = bin.op();
                        let bin_lhs = bin.lhs(); // 可能是不是一个inst的ID
                        let bin_rhs = bin.rhs();

                        let lhs_val = self.dfg().value(bin_lhs);
                        let rhs_val = self.dfg().value(bin_rhs);

                        let lhs_reg: i32;
                        let rhs_reg: i32;

                        let lhs_reg_str: String;
                        let rhs_reg_str: String;

                        match lhs_val.kind() {
                            ValueKind::Integer(int_num) => {
                                load_operation(int_num.value(), bin_lhs, buf, params);
                                lhs_reg = params.current_register;
                            }
                            _ => {
                                lhs_reg = * params.register_data.get(&bin_lhs).unwrap();
                            }
                        }

                        match rhs_val.kind() {
                            ValueKind::Integer(int_num) => {
                                load_operation(int_num.value(), bin_rhs, buf, params);
                                rhs_reg = params.current_register;
                            }
                            _ => {
                                rhs_reg = * params.register_data.get(&bin_rhs).unwrap();
                            }
                        }

                        lhs_reg_str = register_idx_to_name(lhs_reg);
                        rhs_reg_str = register_idx_to_name(rhs_reg);

                        match bin_op {
                            BinaryOp::Add => {
                                writeln!(buf, "  add {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Sub => {
                                writeln!(buf, "  sub {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Mul => {
                                writeln!(buf, "  mul {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Div => {
                                writeln!(buf, "  div {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Mod => {
                                writeln!(buf, "  rem {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Lt => {
                                writeln!(buf, "  slt {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Le => {
                                writeln!(buf, "  sgt {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                                writeln!(buf, "  seqz {}, {}", rhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Gt => {
                                writeln!(buf, "  sgt {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Ge => {
                                writeln!(buf, "  slt {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                                writeln!(buf, "  seqz {}, {}", rhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Eq => {
                                writeln!(buf, "  sub {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                                writeln!(buf, "  seqz {}, {}", rhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::NotEq => {
                                writeln!(buf, "  sub {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                                writeln!(buf, "  snez {}, {}", rhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::And => {
                                writeln!(buf, "  snez {}, {}", lhs_reg_str, lhs_reg_str).unwrap();
                                writeln!(buf, "  snez {}, {}", rhs_reg_str, rhs_reg_str).unwrap();
                                writeln!(buf, "  and {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            BinaryOp::Or => {
                                writeln!(buf, "  snez {}, {}", lhs_reg_str, lhs_reg_str).unwrap();
                                writeln!(buf, "  snez {}, {}", rhs_reg_str, rhs_reg_str).unwrap();
                                writeln!(buf, "  or {}, {}, {}", rhs_reg_str, lhs_reg_str, rhs_reg_str).unwrap();
                            }
                            _ => {}
                        }

                        // 应当释放lhs_reg
                        params.register_usage.insert(lhs_reg, false);
                        
                        params.register_data.insert(inst, rhs_reg);

                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
