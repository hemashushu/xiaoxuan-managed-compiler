# (Practice) Toy Language Interpreter - Rust

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [(Practice) Toy Language Interpreter - Rust](#practice-toy-language-interpreter-rust)
  - [使用方法](#使用方法)
    - [测试](#测试)
    - [编译](#编译)
    - [进入 REPL 模式（交互模式）](#进入-repl-模式交互模式)
    - [运行指定的脚本](#运行指定的脚本)
    - [运行脚本的示例](#运行脚本的示例)
  - [程序示例](#程序示例)
    - [右折叠](#右折叠)
    - [斐波那契数](#斐波那契数)

<!-- /code_chunk_output -->

练习单纯使用 Rust lang 编写简单的 _玩具语言_ 解析器。

> 注：本项目是学习 Rust 随手练习，并无实际用途。

解析器的原理、讲解和代码可以参考：

- 《Writing An Interpreter In Go》 https://interpreterbook.com/
- 《Writing A Compiler In Go》 https://compilerbook.com/
- 《Crafting Interpreters》 https://craftinginterpreters.com/
- 《Building a Parser from scratch》 http://dmitrysoshnikov.com/courses/parser-from-scratch/
- 《Building an Interpreter from scratch》 http://dmitrysoshnikov.com/courses/essentials-of-interpretation/

## 使用方法

### 测试

`$ cargo test`

### 编译

`$ cargo build --release`

### 进入 REPL 模式（交互模式）

`$ cargo run --bin repl`

### 运行指定的脚本

`$ cargo run --bin toy path_to_script_file`

### 运行脚本的示例

`$ cargo run --bin toy examples/01-expression.toy`

如无意外，应该能看到输出 `3`。

## 程序示例

### 右折叠

```js
// 定义（从左往右）折叠函数
//
// * list 是一个数组，比如 [1,2,3]
// * initial 是初始值
// * func 是一个函数，签名为
//   (accumulator, element) -> result

let fold = fn(list, initial, func) {
    let iter = fn(list, accumulator) {
        if (len(list) == 0) {
            accumulator
        } else {
            iter(rest(list), func(accumulator, first(list)));
        }
    };
    iter(list, initial);
};

// 使用折叠函数实现对数组元素求和
let sum = fn(list) {
    fold(
        list,
        0,
        fn(accumulator, element) {
            accumulator + element
        }
    );
};

let n = sum([1, 2, 3, 4, 5]);
puts(n); // 输出 15
```

运行：

`$ cargo run --bin toy examples/03-sum.toy`

如无意外应该能看到输出 15。

### 斐波那契数

```js
// 0、1、1、2、3、5、8、13、21、34、55...

let fib = fn(x) {
    if (x == 0) {
        0
    } else {
        if (x == 1) {
            1
        } else {
            fib(x - 1) + fib(x - 2)
        }
    }
}

fib(9) // 34
```

运行：

`$ cargo run --bin toy examples/04-fib.toy`

如无意外应该能看到输出 34。
