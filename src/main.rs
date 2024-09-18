use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
pub mod generate_asm;
use crate::generate_asm::GenerateAsm;

pub mod ast;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // 我们把生成的Koopa IR放到缓冲区里
    let mut buf: Vec<u8> = Vec::new();

    ast.generate_koopa_ir(&mut buf);

    let koopa_ir = String::from_utf8(buf).unwrap();

    let driver = koopa::front::Driver::from(koopa_ir.clone());
    let program = driver.generate_program().unwrap();

    let mut buf: Vec<u8> = Vec::new();

    program.generate_asm(&mut buf);

    let asm = String::from_utf8(buf).unwrap();

    if mode == "-koopa" {
        // 将 Koopa IR 写入输出文件
        std::fs::write(output, koopa_ir)?;
    } else {
        // 将汇编写入输出文件
        std::fs::write(output, asm)?;
    }


    //println!("{}", output);
    Ok(())
}
