# XiaoXuan Lang Complier Front-end

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [XiaoXuan Lang Complier Front-end](#xiaoxuan-lang-complier-front-end)
  - [使用方法](#使用方法)
    - [测试](#测试)
    - [编译](#编译)
    - [对指定脚本文件进行分词（Lexer）](#对指定脚本文件进行分词lexer)

<!-- /code_chunk_output -->

Rust lang 编写 XiaoXuan Lang 编译器的前端

## 使用方法

### 测试

`$ cargo test`

### 编译

`$ cargo build --release`

### 对指定脚本文件进行分词（Lexer）

`$ cargo run --bin lexer path_to_script_file`

e.g.

`$ cargo run --bin lexer scripts/01-base-expression.xuan`
