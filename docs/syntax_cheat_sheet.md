# Toy 语言语法小抄

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [风格](#风格)
  - [表达式](#表达式)
  - [子表达式](#子表达式)
  - [语句](#语句)
- [字面量](#字面量)
- [复合数据类型](#复合数据类型)
  - [元组](#元组)
  - [结构体](#结构体)
  - [联合体](#联合体)
  - [常量](#常量)
  - [枚举](#枚举)
- [容器](#容器)
  - [列表](#列表)
  - [数组](#数组)
  - [矩阵](#矩阵)
  - [映射](#映射)
- [注释](#注释)
- [其他关键字](#其他关键字)
- [语句](#语句-1)
  - [函数](#函数)
    - [匿名函数](#匿名函数)
    - [函数签名](#函数签名)
    - [参数的类型说明](#参数的类型说明)
  - [模式匹配函数](#模式匹配函数)
  - [关联方法](#关联方法)
  - [特性](#特性)
    - [关联类型](#关联类型)
    - [默认类型](#默认类型)
  - [泛型](#泛型)
    - [泛型特性约束](#泛型特性约束)
  - [类型别名](#类型别名)
  - [命名空间路径](#命名空间路径)
  - [命名空间定义](#命名空间定义)
  - [标注](#标注)
- [表达式](#表达式-1)
  - [标识符定义及赋值表达式](#标识符定义及赋值表达式)
    - [解构/模式匹配](#解构模式匹配)
  - [if 表达式](#if-表达式)
    - [where 子表达式](#where-子表达式)
  - [branch 表达式](#branch-表达式)
    - [where 子表达式](#where-子表达式-1)
  - [for let 表达式块](#for-let-表达式块)
  - [for let 循环表达式](#for-let-循环表达式)
  - [for let .. in 表达式](#for-let-in-表达式)
  - [each .. in 表达式](#each-in-表达式)
  - [模式匹配](#模式匹配)
    - [where 子表达式](#where-子表达式-2)
    - [only 子表达式](#only-子表达式)
    - [into 子表达式](#into-子表达式)
    - [regular 子表达式](#regular-子表达式)
    - [template 子表达式](#template-子表达式)
    - [in 子表达式](#in-子表达式)
    - [to 子表达式](#to-子表达式)
    - [多值匹配](#多值匹配)
    - [嵌套匹配](#嵌套匹配)
  - [函数调用](#函数调用)
    - [普通形式](#普通形式)
    - [中置调用](#中置调用)
    - [前置调用](#前置调用)
    - [匿名函数调用示例](#匿名函数调用示例)
- [符号（按优先级列举）](#符号按优先级列举)
  - [运算符号](#运算符号)
  - [特殊符号](#特殊符号)

<!-- /code_chunk_output -->

## 风格

类 C 风格，跟 Rust 语法比较接近

### 表达式

- 表达式需要一行内写完；
- 完整的表达式、语句后面不需要分号，换行即表示结束；
- 在明显表示有后续内容的符号后面可以换行写，下面是部分这类关键字/符号：
  - = 等号符号后面,
  - : 冒号后面,
  - ( 左括号后面，
  - [ 左中括号后面，
  - { 左花括号后面，
  - + - * / 等二元运算符后面.
- 在表达式或者语句明显尚未完整时，部分表达式是可以换行写：
  - if 各部分的后面,
  - function 的各部分的后面
- 部分符号开头会被认为是承接着上一句，比如：
  - . 点符号开头,
  - ) 括号开头,
  - ] 中括号开头,
  - } 花括号开头,
  - 二元操作符开头.

- "\r\n", "\n" 都是换行符，分号等同于换行符；
- 程序由语句（statement）和表达式（expression）组成，大部分都属于表达式
- 表达式总有返回值
- 多个表达式可以使用花括号 {...} 包围起来，形成表达式块，表达式块的最后一个表达式的值将会作为块的值返回

表达式有：

- let ... = ...
- let ... match ...
- if ... then ... else
- for let ... next
- for ... in
- branch ...
- each let ... in ... [mix let ... in ...] ...
- match ... {...}

### 子表达式

有些关键字属于 `子表达式`，即不能单独存在，仅可用于连接到其他表达式之后，比如

- then
- else
- which
- where
- only
- within
- into
- regular
- template
- to

### 语句

语句无返回值，语句有：

- namespace
- use
- function
- const
- enum
- struct
- union
- trait
- impl
- alias

## 字面量

数字 \d+
逻辑型 true|false
字符 'a'
字符串 "abc"
模板字符串 `abc`
哈希字符串 #abc
正则表达式 /\w+/

## 复合数据类型

### 元组

`(asdf,zxcv) (asdf,) ()`

访问元组的元素：

```
(a,b,c).0
(a,b,c).1
(a,b,c).2
```

### 结构体

```
struct User {
    type_name name
    type_name name
}
```

结构体实例化

```
User {"value", "value"...}
User {name: "value", name: "value"...}
```

或者

`User::new("value", "value", ...)`

实例化嵌入的结构体时，可以省略结构体的名称，比如：

```
struct User {
    Int id,
    String name
    Boolean checked
}

List<User> aa = [{1, "foo", true}, {2, "bar", false}]
```

### 联合体

```
union Name {
      MemberName1 {DataType1 memberName1, DataType2 memberName2, ...}
      MemberName3
}
```

### 常量

```
const Int Code = 123

const Int Code {
    Ok = 123
    Moved = 301
}
```

### 枚举

```
enum Color {
    Red
    Green
    Blue
}
```

## 容器

### 列表

`[1,2,3]`

### 数组

`#[1,2,3]`

### 矩阵

```
![
1,2,3
4,5,6
7,8,9
]
```

### 映射

`{name: value, name: value, ...}`

## 注释

- // 行注释
- /* ... */ 区域注释
- '''
  文档型注释
  '''

## 其他关键字

- empty 表示空函数
- _, _name 下划线或下划线加名称，表示标识符占位符，同时也用于函数部分调用

## 语句

### 函数

```
function name (type name, type name=default) type type_name {
    ...
}

function name (...) type type_name = expression
```

#### 匿名函数

`(type_name name) type type_name => expression`

省略形式

`(name) => expression`

单独一个参数时，可以再省略为

`name => expression`

#### 函数签名

函数的类型（签名）

`sign (type1, type2...) type type_name`

#### 参数的类型说明

```
function name (F f) type Int
    which {
        F: sign (Int x) type Int
    } {
    ...
}
```

多个参数的类型说明

```
function name (T t, F f) type T
    which {
        T: List<String>,
        F: sign (Int x) type Int
    } {
    ...
}
```

### 模式匹配函数

添加了 pattern 的函数，其参数可以使用匹配/解藕表达式（包括 match 表达式 case 关键字后面的各种子表达式）

```js
pattern function test (parse Email email, parse Phone phone) {
    ...
}
```

同名的模式匹配函数会被转换为 branch 结构

### 关联方法

```
assign DataType {
    function name (...) type ... {...}
}
```

### 特性

```
trait Name {
    empty function name (...) type ... {...}
}
```

应用特性

```
assign DataType trait Name {
    ...
}
```

#### 关联类型

```
trait Sequence type ItemType {
    empty function ItemType first(Self s)
}
```

多个关联类型

```
trait Sequence type (ItemType, ItemType) {
    empty function ItemType first(Self s)
}
```

具体化关联类型

```
assign DataType
    trait Sequence
    type ItemType = Type {
...
}
```

#### 默认类型

```
trait Convertable type ItemType=String
    ...
end
```

多项默认类型

```
trait Convertable type (ItemType=String, ItemType=String)
    ...
end
```

### 泛型

`function name<T>(T left, T right) ...`

泛型参数具体化

`let a = name<type_name>(a, b)`

#### 泛型特性约束

```
function max<T> (T left, T right) type T which {
        T: limit Ordered
    } {
    ...
}
```

一个类型多个约束

```
function max<T> (T left, T right) type T which {
    T: limit Display, Ordered
    }
    {
        ...
    }
end
```

### 类型别名

`alias 类型别名 = 源类型`

### 命名空间路径

```
foo::bar
foo::{bar, baz}
```

### 命名空间定义

```
namespace tests {
    ...
}
```

### 标注

`@name(...)`

## 表达式

### 标识符定义及赋值表达式

`let left-hand-side = righ-hand-side`

#### 解构/模式匹配

```
let (a,b,c) = ... // 元组解构
let [a,b] = ... // 列表解构
let #[a,b] = ... // 数组解构
let User{id, name} = ... // 结构体解构
let User{id: user_id, name: user_name } = ...
let Json::String{value} = ... // 枚举解构
```

# let ... match 表达式

`let User{id, name} match user001`

let ... match 表达式 返回一个 Boolean 数值，同时产生相应的变量

通常配合 if 表达式使用，例如 if let ... match then ...，也常用在单元测试中。

### if 表达式

`if ... then ... else ...`

一共三个子表达式，其中 else 可以再省略，每个子表达式也可以是表达式块，其中第一个子表达式需要返回 Boolean 类型的值，比如：

```
if {let a = c * 2; a > b} then
...
else
...
```

if, then, else 关键字后面的表达式都允许换行写

#### where 子表达式

用于补充 `作用域为整个 if 表达式块` 的局部变量，比如

```
if a > 1 where let a = 2 then ...
if a > b where {
    let a = 2
    let b = 1 } then ...
```

### branch 表达式

```
branch {
    case b==0: ...
    case b>a: ...
    default: ...
}
```

#### where 子表达式

branch和 case 关键字后面都可以添加 where 子表达式

branch 后面用于创建当前整个 branch 有效的作用域，比如

```
branch where let a = 2 {
    ...
}
```

case 后面用于创建当前 case 有效的作用域，比如

```
branch {
    case b>a where let a = 1: ...
}
```

### for let 表达式块

`for let 变量 = 初始值 expression`

或者

`for let 变量 = 初始值 {...}`

比起 `{...}` 表达式块，`for let` 表达式块块允许创建一个作用域为该语句块的变量

变量可以是一个元组

`for let (a,b) = (0,1) {...}`

### for let 循环表达式

在语句块里面可以使用 next 关键字让变量的值更新并再次执行一次语句块，因为可以
使用 for let 语句块实现循环结构

```
for let i = 0 if i < 10 then {
    ...
    next i+1
}
```

### for let .. in 表达式

`for let i in [1,2,3] {...}`

for let .. in 返回最后一次执行的语句的值
for let .. in 里面不需要写 next .. 语句

in 后面可以加 mix 关键字，表示混入另一个列表

```
for let i in [1,2,3] mix
    let j in [4,5,6] {
    ...
}
```

### each .. in 表达式

each let i in [1,2,3] {...}

each 返回一个列表

each .. in .. mix .. 表达式，依次从 2 个或多个列表里取出元素

```
let a =
    each let i in [1,2] mix
         let j in [4,5,6] (i,j)
```

返回 [(1,4),(1,5),(1,6), (2,4),(2,5),(2,6)]

跟嵌套多个 `each` 表达式不同，带 mix 的 each 表达式返回的是单一层的列表，而不是嵌套列表。

### 模式匹配

```
match v {
    case a: expression
    case b: {...}
    default: ...
}
```

#### where 子表达式

match 和 case 后面都可以加上 where 子表达式

```
match v where ... {
    case ... where ... : ...,
}
```

#### only 子表达式

也叫守护表达式

```
match v {
    case a only a>0: ...
    case a only {...}: ...
}
```

#### into 子表达式

case 后面可以添加 into 关键字

```
match s {
    case into Email email:
        ...
    case into Phone phone:
        ...
}
```

从数值 s 解析到到 其他枚举类型，实现了 Into<Type> 的源数据都可以使用 into 关键字

#### regular 子表达式

case 后面可以添加 regular 关键字

regular 后面两个表达式，一个表达式必须是正则对象，另一个是标识符列表

```
let s = "foo@domain"

match s
    case regular /^(.+)@(.+)$/ [email, name, domain]:
        writeLineFormat("It's Email: {}", email)
    case regular /^(\\w+):\/\/(.+)$/ [phone, countryCode, number]:
        writeLineFormat("It's phone number: {}", phone)
    default:
        writeLine("Not detected")
end
```

#### template 子表达式

case 后面可以加 template 关键字

```js
match s
    case template `/user/{userName:\w+}`:
        writeLineFormat("Get user {}", userName)
    case template `/user/{userName:\w+}/post/{postId:\d+}`:
        writeLineFormat("Get post {}", postId)
end
```

template `...` 会被转换为 regular

#### within 子表达式

case 后面可以添加 within 关键字

```js
match i
    case within [1..2]:
        ...
```

```js
match c
    case within ['a'..'f']:
        ...
```

关键字 `within` 后面可以是一个 `Range`、一个 `List` 对象，只要是一个拥有 `Exist` 特性的对象都可以。

#### to 子表达式

case 后面可以添加 to 关键字，用于保留被匹配的数据

```
match u {
    case User{name} to u1: ...
}
```

#### 多值匹配

case 后面可以是多个情况的连写，用逗号分隔

```
match c
    case 1,2,3: ...
```

#### 嵌套匹配

```
match u {
    case User{
        name only ...,
        number where ...,
        addr: Addr {
                city within [...],
                ...
            }
        } to u1: ...
}

```

### 函数调用

#### 普通形式

```
name(value1, value2, value3)
name(name1=value1, name2=value2, name3=value3)
```

#### 中置调用

`a :fn_name: b`

#### 前置调用

`!fn_name (data1, arg1, arg2)`

#### 匿名函数调用示例

```
users
    .map((x) => x*2)
    .filter(x => x>3)
```

## 符号（按优先级列举）

### 运算符号

表达式

- `{...}` 表达式组
- `=` 赋值语句
- `>>` 链式调用，类似 `:result_and:`
- `|` 管道

二元运算符

- `:name:` 函数中置调用
- `||` 逻辑或运算
- `&&` 逻辑与运算
- `== !=` 相等比较
- `> >= < <=` 大小比较
- `++` 拼接运算符
- `+ -` 算术运算
- `* /` 算术运算
- `??` 带替代值/默认值的拆封
- `&` 函数组合

一元运算符

- `^` 类型转换操作符，符号必须位于表达式之后
- `?` 拆封，符号必须位于表达式之后
- `-` 负数（跟算术减法符号共用）

修饰符

- `. x[...] x[start:end]` 对象成员、方法调用、索引、框选
- `<..>` 泛型，如 Name<Type>（跟大于号和小于号共用）

基础表达式

- `!` 函数前置调用，符号必须位于标识符之前
- `(...)` 分组、元组
- `[...], #[...], ![...]` 列表、数组、矩阵
- `[start..end] [one,two,..end]` 范围、数列
- 字面量

### 特殊符号

三个点，

位于右手值时，是 `重组运算符`

```
b = [1,2,...a]
b = {n:v, n:v, ...a}
```

位于左手值时，是 `捕获剩余项`

```
let [a,b,...rest] = ...
let {a,b,...rest} = ...
```