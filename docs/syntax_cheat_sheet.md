# XiaoXuan 语言的语法摘要

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [程序的组成](#程序的组成)
  - [表达式](#表达式)
    - [从属表达式](#从属表达式)
  - [语句](#语句)
- [字面量](#字面量)
  - [浮点数](#浮点数)
  - [模板字符串](#模板字符串)
- [常量](#常量)
- [复合数据类型](#复合数据类型)
  - [元组](#元组)
    - [空元组](#空元组)
  - [结构体](#结构体)
  - [联合体](#联合体)
  - [枚举](#枚举)
- [容器](#容器)
  - [列表](#列表)
    - [切片](#切片)
  - [数组](#数组)
  - [范围数列](#范围数列)
  - [矩阵](#矩阵)
  - [映射表](#映射表)
- [注释](#注释)
- [语句](#语句-1)
  - [函数定义](#函数定义)
    - [返回值](#返回值)
    - [函数签名](#函数签名)
    - [参数的类型说明](#参数的类型说明)
    - [空函数](#空函数)
  - [模式函数](#模式函数)
  - [关联函数/方法](#关联函数方法)
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
  - [let 表达式](#let-表达式)
    - [解构/模式匹配](#解构模式匹配)
    - [let 表达式的返回值](#let-表达式的返回值)
  - [do 表达式](#do-表达式)
  - [join 表达式](#join-表达式)
    - [将拼接后的字符串传递给指定函数](#将拼接后的字符串传递给指定函数)
  - [if 表达式](#if-表达式)
    - [if where 从属表达式](#if-where-从属表达式)
  - [branch 表达式](#branch-表达式)
    - [branch where 从属表达式](#branch-where-从属表达式)
  - [for 表达式](#for-表达式)
    - [实现循环](#实现循环)
  - [each 表达式](#each-表达式)
  - [模式匹配](#模式匹配)
    - [match where 从属表达式](#match-where-从属表达式)
    - [only 从属表达式](#only-从属表达式)
    - [in 从属表达式](#in-从属表达式)
    - [@ 变量名](#变量名)
    - [into 从属表达式](#into-从属表达式)
    - [regular 从属表达式](#regular-从属表达式)
    - [template 从属表达式](#template-从属表达式)
    - [嵌套匹配](#嵌套匹配)
  - [匿名函数/子函数](#匿名函数子函数)
  - [函数调用](#函数调用)
    - [普通形式](#普通形式)
    - [中置调用](#中置调用)
    - [前置调用](#前置调用)
    - [匿名函数调用示例](#匿名函数调用示例)
- [符号（按优先级列举）](#符号按优先级列举)
  - [运算符号](#运算符号)
  - [省略符](#省略符)
  - [标识符占位符](#标识符占位符)

<!-- /code_chunk_output -->

## 程序的组成

- 程序由 `语句`（`statement`） 和 `表达式`（`expression`） 组成；
- 语句无返回值；
- 表达式有返回值；
- 除了定义性质的内容（比如结构体定义、常量定义、函数定义）是语句之外，其他都是表达式（比如 `if 表达式`、`for 表达式` 是表达式，而不是语句）；
- 表达式和语句的结束不需要分号，换行即表示结束；
- 分号等同于换行符；
- 表达式需要在一行之内写完；
- 在某些符号或者关键字后面允许换行，然后接着写后续的内容（这些符号或关键字明显表示有后续的内容）：

  - `=` 等号
  - `:` 冒号
  - `,` 逗号
  - `..` 双点号
  - `..=` 闭区间双点号
  - `@` at 号
  - `(` 左括号
  - `[` 左中括号
  - `{` 左花括号
  - 二元运算符的后面
  - 关键字的后面

- 某些符号可以新起一行书写（这些符号明显表示是承接着上一句）：

  - `.` 符号
  - `then`, `else` 等从属表达式
  - `which`, `where`, `only`, `in`, `regular`, `template` 等从属表达式
  - `)` 右括号
  - `]` 右中括号
  - `}` 右花括号

- 使用 `do 表达式` 可以创建一个 `表达式块`，表达式块允许包含一个或多个表达式，表达式会被依次求值（或者说执行），最后一个表达式的值将会作为表达式块的值而返回；
- 在某些关键字（比如 `then`，`else`）后面书写 `do 表达式` 时，可以省略 `do` 关键字而直接写一对花括号，这种表达式块称为 `隠式 do 表达式`，一般直接称为 `表达式块`。

### 表达式

表达式有：

- `do {...}`
- `join {...}`
  <!-- `join to ... {...}` -->
- `let ... = ...`
- `if ... then ... else ...`
  `if let ... = ... then ... else ...`
- `for let ... = ... { next }`
- `each let ... in ... {...}`
- `branch {case, default...}`
- `match ... {case, default...}`

#### 从属表达式

有些关键字属于 `从属表达式`，即不能单独存在，仅可用于连接到其他语句或者表达式之后，比如

- `then`
- `else`
- `which`
- `where`
- `only`
- `in`
- `as`
- `into`
- `regular`
- `template`

### 语句

语句有：

- `namespace`
- `use`
- `function`
- `const`
- `enum`
- `struct`
- `union`
- `trait`
- `impl`
- `alias`

## 字面量

- 整数（含十六进制数、二进制整数）： `123`, `0xbeef`, `0b1001`
- 虚数： `3+4i`
- 比特数： `4'b1010`, `8'xff`
- 逻辑型： `true`，`false`
- 字符： `'a'`, `'\x41'`, `'\u{6587}'`
- 字符串： `"abc"`
- 原始字符串：`"""..."""` ::todo::
- 哈希字符串： `#abc`
- 命名操作符： `:abc:`

### 浮点数

- `1.0`
- `0.123`
- `1e10`
- `1.6e-23`

不支持 "指数值为小数" 的浮点数，比如 `4e0.5`

### 模板字符串

模板字符串使用两个反单引号包围：

`template string`

在模板字符串里可以使用占位符 `{{...}}` 插入表达式，比如：

`template {{place holder}}`

表达式将会被求值，然后连接到模板字符串里。

## 常量

```js
const Int Code = 123
```

常量组：

```js
const Int Code {
    Ok = 123
    Moved = 301
}
```

## 复合数据类型

### 元组

`(foo, bar)`

当元组只有一个元素时，需要在后面添加一个逗号：

`(asdf,)`

使用 `. + 下标` 的方式访问元组的元素：

```js
(a,b,c).0 // a
(a,b,c).1 // b
(a,b,c).2 // c
```

#### 空元组

`()`

空元组是数据类型 `std::Unit` 的字面量。 `Unit` 是一个无成员的结构体，它的值只有自己本身。

### 结构体

```js
struct Name {
    type_name1 member_name1
    type_name2 member_name2
}
```

匿名成员的结构体（也称为元组风格的结构体）：

```js
struct Name(type_name1, type_name2)
```

使用花括号实例化结构体：

```js
User {name1: "value1", name2: "value2"}
```

可以省略成员的名称：

```js
User {"value1", "value2"}
```

使用函数调用的风格实例化结构体：

`User::new("value", "value")`

实例化嵌套的结构体的示例：

```js
struct Addr {
    String city
    String code
}

struct User {
    Int id,
    String name
    Addr addr
}

List<User> aa = [
    {1, "foo", {"sz", 518100}},
    {2, "bar", {"gz", 510600}}
    ]
```

结构体也可以无成员，当定义空成员的结构体时，后面不能加花括号或者括号。

```js
struct Writer
```

<!-- 无成员结构体可以使用花括号 `{...}` 实例化，但无法使用 new 函数实例化，但实例总是相等的（因为没有自己的数据），实例跟结构体本身也可以直接作相等比较：

```js
let m1 = Writer{}
let m2 = Writer{}
assert(m1 == m2)     // true
assert(m1 == Writer) // true
``` -->

无成员结构体无法实例化，这种结构体有且只有一个（同名）实例，比如 `let Writer w = Writer`，第一个 `Writer` 是数据类型，第二个 `Writer` 是实例名称（类似常量）。

```js
let Writer m1 = Writer
assert(m1 == Writer) // true
```

无成员结构体可用于将某些相关函数进行分门别类，作用跟命名空间类似。

### 联合体

联合体的各个成员都是一个结构体

```js
union Name {
      MemberName1 {DataType1 memberName1, DataType2 memberName2, ...}
      MemberName2 (DataType1, DataType2, ...)
      MemberName3
}
```

### 枚举

```js
enum Color {
    Red
    Green
    Blue
}
```

枚举的成员无法自定义其值，枚举也是一种数据类型。

## 容器

### 列表

`[1,2,3]`

#### 切片

`a[from..to]`

示例：

`a[0..10]`

其中 `from` 是闭区间，不可省略，而 `to` 是开区间，且可以省略，例如：

`a[1..]`

如果想让 `to` 为闭区间，需使用 `..=` 符号，比如：

`a[0..=10]`

### 数组

无直接字面量，但可以通过将一个列表字面量赋值给一个数组类型的变量，编译器会自动转换：

`let Array a = [1,2,3]`

### 范围数列

`[from..to]`

示例：

`[0..10]`

其中 `from` 是闭区间，不可省略，而 `to` 是开区间，且可以省略，例如：

`[0..]`

如果想让 `to` 为闭区间，需使用 `..=` 符号，比如：

`[0..=10]`

等差数列

`[first, second..to]`

比如：

`[1,3..9]`

### 矩阵

无直接字面量，但可以通过将一个列表字面量赋值给一个矩阵类型的变量，编译器会自动转换：

```js
let Matrix m =
[
    [1,2,3]
    [4,5,6]
    [7,8,9]
]
```

### 映射表

即 Map：

`{name: value, name: value}`

## 注释

- `//` 行注释
- `/* ... */` 区域注释 ::todo::

文档型注释 ::todo::

```js
  '''
  文档型注释
  '''
```

## 语句

### 函数定义

```js
function name (type name, type name=default) type type_name {
    ...
}

function name (...) type type_name = expression
```

#### 返回值

`type` 用于指示返回值的数据类型，当缺省 `type` 时，返回值数据类型为 `std::Unit`，其值只有 `std::Unit`，字面量为空元组 `()`。

#### 函数签名

函数的类型（函数的签名）可以作为一种数据类型。

`sign (type1, type2...) type type_name`

例如：

`sign (Int x, Int y) type Int`
`sign<T, E> (T x, E y) type T`
`sign (T a, String s) which {T: Int}`

#### 参数的类型说明

```js
function name (F f) type Int
    which {
        F: sign (Int x) type Int
    } {
    ...
}
```

多个参数的类型说明

```js
function name (T t, F f) type T
    which {
        T: List<String>,
        F: sign (Int x) type Int
    } {
    ...
}
```

#### 空函数

`empty` 关键字用于在 `trait` 里定义无具体实现的函数。

```js
empty function name (...) ...
```

### 模式函数

添加了关键字 `pattern` 的函数，其参数可以使用匹配/解藕表达式（包括 match 表达式 case 关键字后面的各种从属表达式）

```js
pattern function test (parse Email email, parse Phone phone) {
    ...
}
```

同名的模式匹配函数会被转换为 branch 结构

### 关联函数/方法

```js
impl DataType {
    function name (...) type ... {...}
}
```

### 特性

```js
trait Name {
    empty function name (...) type ... {...}
}
```

应用特性

```js
impl DataType trait Name {
    ...
}
```

#### 关联类型

```js
trait Sequence type ItemType {
    empty function ItemType first(Self s)
}
```

多个关联类型

```js
trait Sequence type (ItemType1, ItemType2) {
    empty function ItemType first(Self s)
}
```

具体化关联类型

```js
impl DataType
    trait Sequence
    type ItemType = TypeName {
    ...
}

impl DataType
    trait Sequence
    type (ItemType1 = TypeName1, ItemType2 = TypeName2) {
    ...
}
```

#### 默认类型

```js
trait Convertable type ItemType=TypeName {
    ...
}
```

多项默认类型

```js
trait Convertable type (ItemType1=TypeName2, ItemType1=TypeName2) {
    ...
}
```

### 泛型

`function name<T>(T left, T right) ...`

泛型参数具体化

`let a = name<type_name>(a, b)`

#### 泛型特性约束

```js
function max<T> (T left, T right) type T which {
        T: limit Ordered
    } {
    ...
}
```

一个类型多个约束

```js
function max<T> (T left, T right) type T which {
    T: limit Display, Ordered
    }
    {
        ...
}
```

### 类型别名

`alias 类型别名 = 源类型`

例如：

`alias String = Array<Char>`
`alias AddFn = sign (Int x, Int y) type Int`

如果目标类型有泛型，可以声明部分泛型别名，或者全部泛型别名，例如：

`alias OkOnly<T> = Result<T, std::Unit>`
`alias MyResult<T, E> = Result<T, E>`

显然，如果目标类型有部分泛型既没有指定具体类型，也没有泛型别名，那么是语法错误的，例如：

`alias MyResult = Result<T,std::Unit>`

编译器会以找不到名称为 `T` 的数据类型而报错。

### 命名空间路径

```js
foo::bar
foo::{bar, baz}
```

### 命名空间定义

```js
namespace tests {
    ...
}
```

### 标注

`#[name(...)]`

## 表达式

### let 表达式

标识符定义（兼赋值）表达式

`let left-hand-side = righ-hand-side`

例如：

`let a = 10`

#### 解构/模式匹配

left-hand-side 即可以是一个变量，也可以是一个模式匹配表达式，例如：

```js
let (a,b,c) = ... // 元组解构
let [a,b] = ... // 列表解构
//let #[a,b] = ... // 数组解构
let User{id, name} = ... // 结构体解构
let User{id: user_id, name: user_name } = ...
let Json::String{value} = ... // 结构体形式的枚举值解构
let Json::Value(v) = ... // 元组形式的枚举值解构
```

#### let 表达式的返回值

`let 表达式` 返回右手边的值，而不管左手边是一个变量还是模式匹配表达式，例如：

`let User{id, name} = user001`

返回的是 `user001` 的值。

但当配搭 `if` 表达式时，`let ... = ...` 返回的是一个 Boolean 数值，表示是否匹配成功。

### do 表达式

`do {...}`

`do 表达式` 用于创建一个有自己作用域的表达式块。

表达式块包含一个或多个表达式，表达式会依次被求值，表达式块里的最后一个表达式的值将会作为表达式块的值返回。

对于 `then`，`else` 等关键字，如果在其后面书写 `do 表达式`，则可以省略关键字 `do` 而直接写一对花括号，这种表达式块称为 `隠式 Do 表达式`。

注意当一对花括号单独存在时，会被解析为 Map。

### join 表达式

`join {...}`

join 表达式用于连接多个表达式的字符串值，示例：

```js
join {
`<section id="user">`
    `<h1>User List</h1>`
    each let user in users {
        `<div id="{{user.id}}">{{user.name}}</a>`
    }
`</section>`
}
```

join 当中的每个表达式的值都会被执行 `.toString()` 方法取得其值的字符串形式，然后将所有字符串连接起来（无分隔符）。

注意 each 返回的是一个列表（List），列表的 `.toString()` 方法将会对其中所有元素求字符串值，然后将所有字符串连接起来（无分隔符）。

<!-- #### 将拼接后的字符串传递给指定函数

`join to some_function_name {...}`

`some_function_name` 可以是一个函数的名称，也可以是一个值为函数的表达式，函数必须只有一个 String 类型的参数。

```js
join to format(_, "date") {
    ...
}
``` -->

### if 表达式

`if ... then ... else ...`

一共三个从属表达式，其中 else 可以再省略，每个从属表达式也可以是表达式块，其中第一个从属表达式需要返回 Boolean 类型的值，比如：

```js
if {let a = c * 2; a > b} then
    ...
else
    ...
```

if, then, else 关键字后面的表达式都允许换行写

#### if where 从属表达式

用于补充 `作用域为整个 if 表达式块` 的局部变量，比如

```js
if a > 1 where let a = 2 then ...
if a > b where {
    let a = 2
    let b = 1 } then ...
```

### branch 表达式

```js
branch {
    case b==0: ...
    case b>a: ...
    default: ...
}
```

#### branch where 从属表达式

branch 和 case 关键字后面都可以添加 where 从属表达式

branch 后面用于创建当前整个 branch 有效的作用域，比如

```js
branch where let a = 2 {
    ...
}
```

case 后面用于创建当前 case 有效的作用域，比如

```js
branch {
    case b>a where let a = 1: ...
}
```

### for 表达式

`for let 变量 = 初始值 expression`

或者

`for let 变量 = 初始值 {...}`

比起 `do 表达式` 创建的表达式块，`for let` 多了一个作用域为该语句块的变量。

变量可以是一个元组

`for let (a,b) = (0,1) {...}`

跟 `let` 表达式一样，变量也可以是一个模式匹配表达式，例如：

`for let User {name, ...} = user1 {...}`

> `for 表达式` 和 `do 表达式` 的作用类似，只不过前者允许创建一个变量，为了符合习惯，故使用 `for` 关键字。

#### 实现循环

在 `for 表达式` 的语句块里面可以使用 `next` 关键字让变量的值更新并再次执行一次语句块，因此可以使用 `for let 表达式` 实现循环结构：

```js
for let i = 0 if i < 10 then {
    ...
    next i+1
}
```

<!--
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
-->

### each 表达式

`each ... in ... {...}`

`each let i in [1,2,3] {...}`

each 返回一个列表

<!--
each .. in .. mix .. 表达式，依次从 2 个或多个列表里取出元素

```
let a =
    each let i in [1,2] mix
         let j in [4,5,6] (i,j)
```

返回 [(1,4),(1,5),(1,6), (2,4),(2,5),(2,6)]

跟嵌套多个 `each` 表达式不同，带 mix 的 each 表达式返回的是单一层的列表，而不是嵌套列表。
-->

### 模式匹配

```js
match v {
    case a: expression
    case b: {...}
    default: ...
}
```

#### match where 从属表达式

match 后面可以加上 where 从属表达式

```js
match v where ... {
    ...
}
```

case 后面也可以加上 where 从属表达式

```js
match v {
    case Some(a)
        only a.val > avg

}
```

#### only 从属表达式

也叫守护表达式

```js
match v {
    case a only a>0: ...
    case a only {...}: ...
}
```

#### in 从属表达式

case 后面可以添加 in 关键字

```js
match i {
    case in [1..2]:
        ...
}
```

```js
match c {
    case in ['a'..'f']:
        ...
}
```

关键字 `in` 后面可以是一个 `Range`、一个 `List` 对象，只要是一个拥有 `Exist` 特性的对象都可以。

#### @ 变量名

case 后面可以添加一个标识符然后接着符号 `@`，用于保留被匹配的数据

```js
match u {
    case u1 @ User{name}: ...
}
```

#### into 从属表达式

case 后面可以添加 into 关键字

```js
match s {
    case into Email email:
        ...
    case into Phone phone:
        ...
}
```

从数值 s 解析到到 其他枚举类型，实现了 Into<Type> 的源数据都可以使用 into 关键字

#### regular 从属表达式

case 后面可以添加 regular 关键字

regular 后面两个表达式，一个表达式正则表达式的字符串字面量，另一个是标识符列表

```js
let s = "foo@domain"

match s {
    case regular "^(.+)@(.+)$" (email, name, domain):
        writeLineFormat("It's Email: {}", email)
    case regular "^(\\w+):\/\/(.+)$" (phone, countryCode, number):
        writeLineFormat("It's phone number: {}", phone)
    default:
        writeLine("Not detected")
}
```

#### template 从属表达式

case 后面可以加 template 关键字，关键字后面跟着一个字符串字面量，字符串里可以添加占位符 `{...}`，占位符需要写上该占位符的名称，以及正则表达式。如果省略正则表达式，默认为 `\w+`

```js
match s {
    case template "/user/{userName:\w+}":
        writeLineFormat("Get user {}", userName)
    case template "/user/{userName:\w+}/post/{postId:\d+}":
        writeLineFormat("Get post {}", postId)
}
```

template 会被转换为 regular

#### 嵌套匹配

```js
match u {
    case User{
        name only ...,
        number only ... where ...,
        addr: Addr {
                city in [...],
                ...
            }
        }: ...
}

```

### 匿名函数/子函数

`fn (type_name param_name) type type_name = ...`
`fn (type_name param_name) type type_name {...}`

省略参数和返回值数据类型的形式：

`fn (param1, param2) = ...`

无参数时的形式：

`fn () = ...`

单独一个参数时，可以再省略为：

`fn param_name = ...`

比如：

```js
let s = [1,2,3]
    .map(fn x = x.to_string())
    .join("")
```

### 函数调用

#### 普通形式

```js
name(value1, value2, value3)
name(name1=value1, name2=value2, name3=value3)
```

#### 中置调用

`a :fn_name: b`

#### 前置调用

`!fn_name (data1, arg1, arg2)`

#### 匿名函数调用示例

```js
users
    .map(fn (x) = x*2)
    .filter(fn x = x>3)
```

## 符号（按优先级列举）

### 运算符号

表达式

- `do {...}` 表达式组
- `=` 赋值语句
- `|` 管道

二元运算符

- `:name:` 函数中置调用
- `||` 逻辑或运算
- `&&` 逻辑与运算
- `== !=` 相等比较
- `> >= < <=` 大小比较
- `>>` 链式调用，类似 `:result_and:`
- `++` 拼接运算符
- `+ -` 算术运算
- `* /` 算术运算
- `??` 带替代值/默认值的拆封
- `&` 函数组合（从右向左结合）

一元运算符

- `^` 类型转换操作符，符号必须位于表达式之后
- `?` 拆封，符号必须位于表达式之后
- `-` 负数（跟算术减法符号共用）

修饰符

- `.`， `x[...]`， `x[from..to]` 对象成员、方法调用、索引、框选
- `<...>` 泛型，如 `Name<Type>` （跟大于号和小于号共用）

基础表达式

- `!` 函数前置调用，符号必须位于标识符之前
- `(...)` 分组
- `(... ,)` 元组
<!-- - `[...], #[...], ![...]` 列表、数组、矩阵 -->
- `[...]` 列表
- `{...}` 映射表
- `[from..to] [one,two,..to]` 范围数列

### 省略符

由三个点 `...` 组成的省略符，位于右手值时，表示 `重组运算符`:

```js
b = [1,2,...a]
b = {n:v, n:v, ...a}
```

位于左手值时，表示 `捕获剩余项`：

```js
let [a,b,...rest] = ...
let {a,b,...rest} = ...
```

### 标识符占位符

`_` 表示接收返回值但不保留其值：

`let _ = add(1,2)`
`let (id, _) = user1`

`_` 同时也用于构建部分调用函数：

```js
function add(Int a, Int b) {...}

let inc_ten = add(10, _)
inc_ten(5) // return 15
```

`_name` 表示命名标识符占位符，在构建部分调用函数时，可以重新为新参数命名：

```js
function draw(Point point, Int width, Color color) {...}

let draw_thin_line = draw(_p, 1, _c)
draw_thin_line({10,20}, Color::Red)
draw_thin_line(p: {10,20}, c: Color::Red)

```