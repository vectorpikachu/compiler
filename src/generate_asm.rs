// 根据内存形式 Koopa IR 生成汇编
use koopa::ir::ValueKind;
use std::io::Write;
use koopa::ir::*;
use std::collections::HashMap;

pub struct GenerateAsmParams {
    pub current_register: i32, // 当前使用的register编号
    pub stack_bytes: i32,
    pub register_data : HashMap<Value, i32>, // 指令对应的寄存器
    pub register_usage : HashMap<i32, bool>, // 寄存器的使用情况
    pub stack_state : HashMap<Value, i32>, // 对应的变量和偏移量
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
        // 首先计算出是否需要在栈上分配空间
        for (&_bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                let inst_data = self.dfg().value(inst);
                // 计算出需要分配栈空间的指令.
                if inst_data.ty().is_unit() == false {
                    params.stack_state.insert(inst, params.stack_bytes);
                    params.stack_bytes += 4;
                }
            }
        }
        // 首先计算出prologue, 对齐到16字节
        if params.stack_bytes % 16 != 0 {
            params.stack_bytes += 16 - params.stack_bytes % 16;
        }
        if params.stack_bytes > 0 {
            if params.stack_bytes <= 2048 {
                writeln!(buf, "  addi sp, sp, -{}", params.stack_bytes).unwrap();
            } else {
                writeln!(buf, "  li t0, {}", params.stack_bytes).unwrap();
                writeln!(buf, "  sub sp, sp, t0").unwrap();
            }
        }
        // ...
        // 访问基本块
        for (&bb, node) in self.layout().bbs() {
            // 访问指令列表
            let bb_name = self.dfg().bb(bb).name().as_ref().unwrap().replace("%", "");
            writeln!(buf, "{}:", bb_name).unwrap();
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
                        // 在这里实现函数的epilogue
                        let ret_value_data = ret_value.value().unwrap();
                        match self.dfg().value(ret_value_data).kind() {
                            ValueKind::Integer(int_num) => {
                                writeln!(buf, "  li a0, {}", int_num.value()).unwrap();
                            }
                            _ => {
                                let delta = * params.stack_state.get(&ret_value_data).unwrap();
                                load_and_save("lw".to_string(), 7, delta, buf);
                                if params.stack_bytes > 0 {
                                    if params.stack_bytes <= 2048 {
                                        writeln!(buf, "  addi sp, sp, {}", params.stack_bytes).unwrap();
                                    } else {
                                        writeln!(buf, "  li t0, {}", params.stack_bytes).unwrap();
                                        writeln!(buf, "  add sp, sp, t0").unwrap();
                                    }
                                }
                            }
                        }
                        //let reg = params.register_data.get(&ret_value_data);
                        //match reg {
                        //    Some(reg_idx) => {
                        //        let reg_str = register_idx_to_name(*reg_idx);
                        //        writeln!(buf, "  mv a0, {}", reg_str).unwrap();
                        //    }
                        //    None => {
                        //        match self.dfg().value(ret_value_data).kind() {
                        //            ValueKind::Integer(int_num) => {
                        //                writeln!(buf, "  li a0, {}", int_num.value()).unwrap();
                        //            }
                        //            _ => {}
                        //        }
                        //    }
                        //}
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
                                // load_operation(int_num.value(), bin_lhs, buf, params);
                                // lhs_reg = params.current_register;
                                writeln!(buf, "  li t0, {}", int_num.value()).unwrap();
                                lhs_reg = 0;
                            }
                            _ => {
                                // lhs_reg = * params.register_data.get(&bin_lhs).unwrap();
                                // 直接从栈里面读出来
                                let delta = * params.stack_state.get(&bin_lhs).unwrap();
                                load_and_save("lw".to_string(), 0, delta, buf);
                                lhs_reg = 0;
                            }
                        }

                        match rhs_val.kind() {
                            ValueKind::Integer(int_num) => {
                                // load_operation(int_num.value(), bin_rhs, buf, params);
                                // rhs_reg = params.current_register;
                                writeln!(buf, "  li t1, {}", int_num.value()).unwrap();
                                rhs_reg = 1;
                            }
                            _ => {
                                // rhs_reg = * params.register_data.get(&bin_rhs).unwrap();
                                let delta = * params.stack_state.get(&bin_rhs).unwrap();
                                load_and_save("lw".to_string(), 1, delta, buf);
                                rhs_reg = 1;
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
                        let delta = * params.stack_state.get(&inst).unwrap();
                        load_and_save("sw".to_string(), rhs_reg, delta, buf);
                        // 应当释放lhs_reg
                        params.register_usage.insert(lhs_reg, false);
                        
                        params.register_data.insert(inst, rhs_reg);

                    }
                    ValueKind::Alloc(_alloc) => {
                        // Do nothing
                        // writeln!(buf, "  addi sp, sp, -4").unwrap();
                    }
                    ValueKind::Store(store) => {
                        let store_value = store.value();
                        let store_value_data = self.dfg().value(store_value);
                        let store_dest = store.dest();
                        // let store_dest_data = self.dfg().value(store_dest);
                        match store_value_data.kind() {
                            ValueKind::Integer(int_num) => {
                                writeln!(buf, "  li t0, {}", int_num.value()).unwrap();
                            }
                            _ => {
                                // 这里是对上一个指令的指针
                                let delta = * params.stack_state.get(&store_value).unwrap();
                                load_and_save("lw".to_string(), 0, delta, buf);
                            }
                        }
                        // store_dest 不可能是一个整数
                        let delta = * params.stack_state.get(&store_dest).unwrap();
                        load_and_save("sw".to_string(), 0, delta, buf);
                    }
                    ValueKind::Load(load) => {
                        let load_src = load.src();
                        let delta = * params.stack_state.get(&load_src).unwrap();
                        load_and_save("lw".to_string(), 0, delta, buf);
                        let delta = * params.stack_state.get(&inst).unwrap();
                        load_and_save("sw".to_string(), 0, delta, buf);
                    }
                    ValueKind::Branch(branch) => {
                        let cond = branch.cond();
                        let true_dst = branch.true_bb();
                        let false_dst = branch.false_bb();
                        let cond_val = self.dfg().value(cond);
                        let cond_str: String;
                        let true_name = self.dfg().bb(true_dst).name().as_ref().unwrap()
                                                    .replace("%", "");
                        let false_name = self.dfg().bb(false_dst).name().as_ref().unwrap()
                                                     .replace("%", "");
                        match cond_val.kind() {
                            ValueKind::Integer(i) => {
                                if i.value() != 0 {
                                    writeln!(buf, "  j {}", true_name).unwrap();
                                } else {
                                    writeln!(buf, "  j {}", false_name).unwrap();
                                }
                                continue;
                            }
                            _ => {
                                let delta = * params.stack_state.get(&cond).unwrap();
                                load_and_save("lw".to_string(), 0, delta, buf);
                                cond_str = register_idx_to_name(0);
                            }
                        }
                        // 去掉名字里的%号
                        
                        writeln!(buf, "  bnez {}, {}", cond_str, true_name).unwrap();
                        writeln!(buf, "  j {}", false_name).unwrap();
                    }
                    ValueKind::Jump(jump) => {
                        let target = jump.target();
                        let target_name = self.dfg().bb(target).name().as_ref().unwrap()
                                                      .replace("%", "");
                        writeln!(buf, "  j {}", target_name).unwrap();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn load_and_save(mode: String, target_reg: i32, delta: i32, buf: &mut Vec<u8>) {
    let reg_str = register_idx_to_name(target_reg);
    if delta >= 2048 {
        writeln!(buf, "  li t0, {}", delta).unwrap();
        writeln!(buf, "  add t0, sp, t0").unwrap();
        writeln!(buf, "  {} {}, t0", mode, reg_str).unwrap();
    }
    else {
        writeln!(buf, "  {} {}, {}(sp)", mode, reg_str, delta).unwrap();
    }
}