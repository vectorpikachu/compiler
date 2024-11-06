# Compiler

## 安装Docker

```shell
docker pull maxxing/compiler-dev
```

## 整体的架构

sysy.lalrpop 放着词法分析的.

运行的命令行：
```shell
cargo run -- -koopa hello.c -o hello.koopa
```

现在加入gitlab.

```shell
autotest -koopa -s lv3 /root/compiler
```

如果放的是%n, 实际上是一个Value, 否则可以在dfg中找到.

```
docker run -it --rm -v D:\HuaweiMoveData\Users\平面向皮卡丘\Desktop\compilers\compiler:/root/compiler maxxing/compiler-dev \
  autotest -riscv -s lv3 /root/compiler
```

todo: 解决multiple returns 的问题

