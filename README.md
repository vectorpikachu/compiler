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

