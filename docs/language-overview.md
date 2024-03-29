# XiaoXuan 语言摘要

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [程序的组成](#程序的组成)
  - [表达式](#表达式)
  - [语句](#语句)
- [字面量](#字面量)
  - [浮点数](#浮点数)
  - [字符串](#字符串)
  - [原样字符串](#原样字符串)
  - [模板字符串](#模板字符串)
  - [正则表达式](#正则表达式)
  - [哈希字符串](#哈希字符串)
- [常量](#常量)
- [复合数据类型](#复合数据类型)
  - [元组](#元组)
  - [结构体](#结构体)
    - [实例化结构体](#实例化结构体)
    - [函数式实例化结构体](#函数式实例化结构体)
    - [结构体实例化设计模式](#结构体实例化设计模式)
    - [使用类型转换操作符 `^` 从元组转换为结构体](#使用类型转换操作符--从元组转换为结构体)
    - [无成员结构体](#无成员结构体)
  - [联合体](#联合体)
  - [枚举](#枚举)
- [容器](#容器)
  - [列表](#列表)
    - [切片](#切片)
    - [范围数列](#范围数列)
    - [栈](#栈)
    - [矩阵](#矩阵)
  - [映射表](#映射表)
- [注释](#注释)
- [表达式元素](#表达式元素)
- [语句](#语句-1)
  - [函数定义](#函数定义)
    - [返回值类型 `result` 关键字](#返回值类型-result-关键字)
    - [函数参数的类型说明 `type 从属表达式`](#函数参数的类型说明-type-从属表达式)
    - [参数的默认值](#参数的默认值)
  - [函数签名](#函数签名)
  - [空函数](#空函数)
  - [永不返回函数](#永不返回函数)
  - [关联函数/方法](#关联函数方法)
  - [接口](#接口)
    - [关联类型](#关联类型)
    - [默认类型](#默认类型)
  - [泛型](#泛型)
    - [泛型函数](#泛型函数)
    - [泛型结构体](#泛型结构体)
    - [泛型接口约束](#泛型接口约束)
  - [类型别名](#类型别名)
    - [命名空间路径](#命名空间路径)
  - [导入标识符](#导入标识符)
  - [标注](#标注)
- [表达式](#表达式-1)
  - [let 表达式](#let-表达式)
    - [let 模式匹配](#let-模式匹配)
    - [let 表达式的返回值](#let-表达式的返回值)
  - [do 表达式](#do-表达式)
  - [if 表达式](#if-表达式)
    - [if where 从属表达式](#if-where-从属表达式)
  - [if let 表达式](#if-let-表达式)
  - [branch 表达式](#branch-表达式)
    - [branch where 从属表达式](#branch-where-从属表达式)
    - [branch case where 从属表达式](#branch-case-where-从属表达式)
  - [for 表达式](#for-表达式)
    - [省略入参的](#省略入参的)
    - [for let 多个参数](#for-let-多个参数)
    - [实现循环](#实现循环)
    - [for 表达式的返回值](#for-表达式的返回值)
  - [for let ... in ... 表达式](#for-let--in--表达式)
  - [match 表达式 （模式匹配）](#match-表达式-模式匹配)
    - [match where 从属表达式](#match-where-从属表达式)
    - [match case 变量部分](#match-case-变量部分)
    - [match case 从属表达式](#match-case-从属表达式)
      - [where 从属表达式](#where-从属表达式)
      - [only 从属表达式](#only-从属表达式)
    - [模式表达式](#模式表达式)
      - [in 模式表达式](#in-模式表达式)
      - [into 模式表达式](#into-模式表达式)
      - [regular 模式表达式](#regular-模式表达式)
      - [template 模式表达式](#template-模式表达式)
    - [嵌套匹配](#嵌套匹配)
  - [模式函数](#模式函数)
  - [fn 表达式（匿名函数/子函数）](#fn-表达式匿名函数子函数)
  - [函数调用](#函数调用)
    - [普通形式](#普通形式)
    - [中置调用](#中置调用)
    - [前置调用](#前置调用)
    - [匿名函数调用示例](#匿名函数调用示例)
- [符号（按优先级列举）](#符号按优先级列举)
  - [运算符号](#运算符号)
  - [省略符](#省略符)
  - [标识符占位符](#标识符占位符)
- [宏](#宏)

<!-- /code_chunk_output -->

## 程序的组成

- 程序由 `语句`（`statement`） 和 `表达式`（`expression`） 组成； **语句构成了程序的结构，表达式构成了程序的内容。**
- 语句无返回值；
- 表达式有返回值；
- 除了定义性质的内容（比如结构体定义、常量定义、函数定义）是语句之外，其他都是表达式（跟传统的语言不太一样，XiaoXuan lang 的 `let`, `if`, `for` 等都是表达式，而不是语句）；

- 关于换行符
  - 表达式和语句的结束不需要分号，换行即表示结束；
  - 分号 **等同于** 换行符；
  - 表达式不必在一行之内写完；
    某些符号的后面允许换行，然后接着写后续的内容：
    - `=` 等号
    - `:` 冒号
    - `,` 逗号
    - `..` 双点号
    - `..=` 闭区间双点号
    - `@` at 号
    - `(` 左括号
    - `[` 左中括号（方括号）
    - `{` 左花括号
    - 二元运算符
    - 关键字

    某些符号和关键字之前也可以新起一行写：
    - `)` 右括号
    - `]` 右中括号（方括号）
    - `}` 右花括号
    - `.` 点号
    - 各种从属表达式之前

  简单来说，在一个表达式或者语句未完整之前，且不会引起歧义的情况下，都允许换行。

- 表达式块
  - 使用 `do 表达式` 可以创建一个 `表达式块`，表达式块允许包含一个或多个表达式，表达式会被依次求值（或者说执行），最后一个表达式的值将会作为表达式块的值而返回；
  - 在某些关键字（比如 `then`，`else`， `where`, `case/default 冒号之后`）后面书写 `do 表达式` 时，可以省略 `do` 关键字而直接写一对花括号，这种表达式块称为 `隠式 do 表达式`，一般直接称为 `表达式块`。

### 表达式

表达式有：

- `do {...}`
<!-- - `join {...}` 无需专门设计一个关键字，使用宏来代替此功能即可 -->
- `let ~~~ = ...`
- `if ___ then ___ else ___`
- `if let ~~~ = ... then ___ else ___`
- `for let ~~~ = ... ___`
- `for let ~~~ in ... ___`
- `fn ...`
- `branch {...}`
- `match ... {...}`

注：

- 三个点 `...` 表示任意表达式；
- 三个下划线 `___` 表示任意表达式，或者一对花括号包围起来的表达式块（也叫 `隠式 do 表达式`）；
- 三个波浪先 `~~~` 表示 `左手边表达式`（也叫 `模式表达式`）；
- 花括号属于固定语法的一部分，比如关键字 `do` 后面必须带上一对花括号，而不能省略。

### 语句

语句有：

<!-- - `namespace` -->
- `use`
- `function`
- `const`
- `enum`
- `struct`
- `union`
- `interface`
- `implement`
- `type`

## 字面量

- 整数（含十六进制数、二进制整数）： `123`, `0xbeef`, `0b1001`, `123@uint64`
- 虚数： `3+4i`
- 比特数： `4'b1010`, `8'xff`
- 逻辑型： `true`，`false`
- 字符： `'a'`, `'\x41'`, `'\u{6587}'`
- 命名操作符： `:abc:`

### 浮点数

- `1.0`
- `0.123`
- `1e10`
- `1.6e-23`

不支持 "指数值为小数" 的浮点数，比如 `4e0.5`

### 字符串

`"abc"`

字符串以双引号开始，直到另一个双引号为止（不包括 `\"`），允许多行。

注：字符串对象实际上是一个字节列表的切片（切片指向一个伴生结构体，即附带一段原始数据的结构体，该结构体结构为 {[ref_count], long pointer, long len}，其中 pointer 指向原始字符串，即字节数组。原始字符串被分配到 data section 或者 heap 里。原始字符串是“值”对象（即变量赋值或者传参都会引起数据复制），可以使用 r"abc" 来构建并分配到栈里，不过一般很少这样使用。）

### 原样字符串

（“原样字符串” 不是 “原始字符串”，原样字符串是指保持字面上的格式的字符串）

`"""..."""`

以三双引号开始，直到另一个三双引号为止，允许多行，其中字符不转义。 <!-- 考虑取消此接口，使用 trim_prefix 函数加编译优化即可 >> 如果原始字符串以换行符开始，则多行文本当中的共同前缀空白会被自动截断 -->，例如：

```
"""
  a
 b
c
"""
```

上面三行 <!-- 的共同前缀空白是 1 个空格，所以每一个行都会被截去一个空格，--> 最终字符串的值为：`↩️🈳🈳a↩️🈳b↩️c↩️`。

### 模板字符串

模板字符串使用两个反单引号包围：

`template string`

在模板字符串里可以使用占位符 `{{...}}` 插入表达式，比如：

`my name is {{name}}, i'm {{ 6 * 7 }} years old.`

表达式将会在运行时求值，然后连接到模板字符串里。

### 正则表达式

```js
let r = /foo/
```

### 哈希字符串

格式如下：`#abc` 或者 `h"abc"`

字面量会自动生成为对象 Hash{long value, string str}

## 常量

```js
const int Code = 123
```

常量组：

```js
const int Code {
    Ok = 123
    Moved = 301
}
```

## 复合数据类型

### 元组

`(foo, bar)`

元组当中的项目可以换行写，但项目之间的逗号 **不能省略**：

```js
(
    foo,
    bar,
)
```

当元组只有一个元素时，需要在后面添加一个逗号：

`(asdf,)`

使用 `. + 下标` 的方式访问元组的元素：

```js
(a,b,c).0 // == a
(a,b,c).1 // == b
(a,b,c).2 // == c
```

注：元组是值对象，是匿名的原始结构体。

<!--
#### 空元组

`()`

空元组是数据类型 `std::Empty` 的唯一值 `std::Empty` （注：值和类型同名） 的字面量。 `std::Empty` 是一个无成员的结构体，它的值只有自己本身。
-->

### 结构体

```js
struct Name {
    type_name1 member_name1
    type_name2 member_name2
}
```

使用 `点号` 可以访问结构体的成员，比如：

`user.id`
`user.name`

点号前面也可以是一个用括号包围的表达式或表达式块，只需返回的是一个结构体示例即可：

`(getUser("001")).id`

也可以连续访问成员，比如：

`user.addr.city`

匿名成员的结构体（也称为元组风格的结构体）：<!-- 考虑取消此接口，如果需要使用括号实例化结构体，使用结构体的构造函数即可 -->

```js
struct Name(type_name1, type_name2)
```

使用 `点号` + `数字` 的方式访问匿名成员结构体的成员，比如

```js
data.0 // 第一个成员
data.1 // 第二个成员
```

#### 实例化结构体

使用花括号实例化结构体：

`let user = User {name1: "value1", name2: "value2"}`

或者

`let user = User {name2: "value2", name1: "value1"}`

键值的出现顺序不重要。

如果上下文存在一个变量刚好跟结构体的某个成员同名，则可以省略书写 `key name`：

```js
let id = 123
let name = "foo"
let user = User {id, name}
```

同样的，键值的出现顺序不重要。

#### 函数式实例化结构体

`let user = User("value1", "value2")`

参数的顺序必须跟结构体成员的顺序一致，且不能省略。

#### 结构体实例化设计模式

可以为结构体会添加一个 `new` 方法，其代码如下：

```js
implement User {
    function new(int id, string name) result User {
        User {id, name}
    }
}
```

然后通过调用这个 `new` 方法实例化结构体：

`let user = User::new(123, "foo")`
`let user = User::new(id=123, name="foo")`

#### 使用类型转换操作符 `^` 从元组转换为结构体

`let User user = (123, "foo")^`

通过元组类型转换，有时可以简化嵌套的结构体的实例化过程：

```js
struct Addr {
    string city
    string code
}

struct User {
    int id,
    string name
    Addr addr
}

// 普通方式
List<User> a1 = [
    User {id: 1, name: "foo", Addr {city: "sz", code: 518100}},
    User {id: 2, name: "bar", Addr {city: "gz", code: 510600}},
    ]

// 使用元组转换方式
List<User> a2 = [
    (1, "foo", ("sz", 518100)^)^,
    (2, "bar", ("gz", 510600)^)^
    ]
```

注：元组与结构体的相互转换是语言级的支持，而不是通过 `Into` 特质实现的。不过用法跟 Into 接口一样。

注：结构体对象是一个内部结构体，{[ref_count], long pointer}，其中 pointer 指向原始结构体，原始结构体分配在 data section 或者 heap 中。可以使用 raw_struct {...} 来定义原始结构体，不过一般很少这样使用。

#### 无成员结构体

结构体也可以无成员，当定义空成员的结构体时，后面的花括号不能省略

```js
struct Writer {}
```

无成员结构体的实例化：

<!--
无成员结构体无法实例化（注：这点跟 rust lang 不同），这种结构体有且只有一个（同名）实例，比如 `let Writer w = Writer`，第一个 `Writer` 是数据类型，第二个 `Writer` 是实例名称（类似常量）。
-->

无成员结构体的实例化语句同样不能省略后面的花括号或者括号：

```js
let Writer m1 = Writer{}
let Writer m2 = Writer()
```

如果无成员的结构体派生了 "Equal" 接口，当比较它们的实例时，结果是 true

```js
#[derive(Debug)]
struct User{}

let Writer m1 = Writer{}
let Writer m2 = Writer{}
assert_true(m1 == m2) // true
```

无成员结构体一般用于实现接口。

### 联合体

联合体的各个成员都是一个结构体

```js
union Name {
      MemberName1 {DataType1 memberName1, DataType2 memberName2, ...}
      MemberName3

      // 不支持这种类型 MemberName2 (DataType1, DataType2, ...)
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

TODO:: 枚举的数据类型和成员值的指定

## 容器

### 列表

`[1,2,3]`

列表当中的项目可以换行写，但项目之间的逗号 **不能省略**：

```js
[
    1,
    2,
    3
]
```

使用索引访问列表元素

- `a[0]`
- `a[10]`

中括号前也可以是一个用括号包围起来的表达式或表达式块，只需返回值是一个列表即可。

索引值也可以是一个表达式，比如：

`a[1+2]`

也允许连续访问索引，比如：

`a[1][2]`

#### 切片

`a[from..to]`

示例：

`a[0..10]`

其中 `from` 是闭区间，不可省略，而 `to` 是开区间，且可以省略，例如：

`a[1..]`

如果想让 `to` 为闭区间，需使用 `..=` 符号，比如：

`a[0..=10]`

注：列表对象实际上是一个切片，切片指向一个列表伴生结构体，该结构体的附属数据是一个原始数组，它们的关系如下：

```text
slice {[ref_count],
       pointer, ---------> array {[ref_count],
       offset,                    pointer, --------> raw_array [...]
       length                     length,
      }                           item_size,
                                  deref_func,
                                 }
```

字符串实际是类型为 u8 的列表。另外结构体的伴生结构体跟列表的不一样，结构体的如下：

```text
struct {[ref_count],
        pointer, ---------> raw_struct {...}
        length,
        deref_func,
}
```

<!--
#### 数组  考虑使用 Buffer 代替

无直接字面量，但可以通过将一个列表字面量赋值给一个数组类型的变量，编译器会自动转换：

`let Array a = [1,2,3]`
-->

#### 范围数列

`[from..to]`

示例：

`[0..10]`

其中 `from` 是闭区间，不可省略，而 `to` 是开区间，且可以省略，例如：

`[0..]`

如果想让 `to` 为闭区间，需使用 `..=` 符号，闭区间不能省略 `to` 部分，比如：

`[0..=10]`

等差数列

`[first, second..to]`

比如：

`[1,3..9]`

#### 栈

列表无法添加删除元素（列表的切分，删除头或尾的元素等操作实际上是通过切片来完成的），对于需要添加/删除元素的容器，可以使用 “栈”，一个单向链表。栈的元素构建完毕后，可以通过 to_list 方法将它转换为列表。

“栈” 无字面量，但可以通过宏 stack![0,1,2,...] 来简化构建语句。

#### 矩阵

矩阵是 “二维列表” 的别称，字面量如下：

<!-- 无直接字面量，但可以通过将一个列表字面量赋值给一个矩阵类型的变量，编译器会自动转换： -->

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

```js
{
    name1: value1
    name2: value2
}
```

映射表的每一项使用 `key: value` 这样的格式书写，注意跟元组和列表的项目之间必须加上逗号不同，`映射表` 项目之间可以省略分隔逗号，项目之间使用 `换行符` 也可以分隔。

> 同样使用花括号作为主体的 `多行格式的映射表`/`where`/`branch`/`match`/`do`/`join` 结构格式保持一致。

```js
{
    name1: value1,
    name2: value2,
}

```

有时为了紧凑书写，也可以将映射表的多个项目写在同一行，这时项目之间就必须加上逗号分隔：

```js
{name1: value1, name2: value2}
```

当 key 的名称跟 value 的变量名称相同时，也可以省略 ": value" 部分，例如：

```js
let number = 123
let username = "foo"
let user1 = {number: number, username: username}
let user2 = {number, username}
```

上例的后两句的作用是一样的，即在实例化映射表时，如果省略了 ": value" 部分，即只有 "name"，其实是表达式 "name: name" 的语法糖。

映射表的 Key 是 hash_string 类型，使用方法读取项目值的示例如下：

`user_001.get(#id)`
`user_001.get(#name)`

也可以使用跟访问列表索引的方式来访问映射表的元素：

`user_001[#id]`
`user_001[#name]`

注意，映射表不支持类似 JavaScript 使用点号读取项目值，比如 `user_001.id` 这样是不支持的。

中括号前面也可以是一个用括号包围的表达式或表达式块，只需返回值是一个映射表即可：

`(get_map("001"))[#id]`

也允许连续访问元素：

`user_001[#addr][#street]`

## 注释

- `/* ... */` （区域）注释
- `//` 行注释
- 文档型注释

```js
    '''
    文档型注释
    '''
    function name() result int {
        ...
    }
```

## 表达式元素

表达式元素包括可以作为赋值运算符 `左手边值` 的对象，包括：

- 元组
- 列表
- 映射表
- 标识符

以及各种数值类的字面量。

## 语句

### 函数定义

`function name (data_type param_name, data_type param_name=default) result data_type = ...`
`function name (data_type param_name, data_type param_name=default) result data_type {...}`

#### 返回值类型 `result` 关键字

`result` 用于指示返回值的数据类型，当缺省 `result` 时，表示该函数无返回值。

注，在运行时内部，如果将一个无返回值的函数的调用结果赋值给一个变量，则会产生语法错误。

#### 函数参数的类型说明 `type 从属表达式`

```js
function name (F f) result int
    type F = signature (int x) result int {
    ...
}
```

多个参数的类型说明

```js
function name (T t, F f) result T
    type {
        T = List<string>,
        F = signature (int x) result int
    } {
    ...
}
```

多个参数说明行的末尾的逗号是可选的，

> 同样使用花括号作为主体的 `多行格式的映射表`/`where`/`branch`/`match`/`do`/`join` 结构格式保持一致。
> 注：函数的 type 等从属表达式的顺序不重要。

不支持函数重载（即同名函数），但支持 “模式匹配函数” 这种看起来像是函数重载的格式。

<!--

#### 函数重载

名称相同，返回值相同，但参数列表不同的函数称为函数重载。

```js
function fun1 (int a) result int {
    ...
}

function fun2 (int a, int b) result int {
    ...
}
```
-->

#### 参数的默认值

提供了默认值的参数为 `可选参数`

```js
function name(int a, int b = 100) ...
```

可选参数必须排在参数列表的后面，允许多个可选参数。

```js
function draw(int width, Style style = Style::Solid, Color color = Color::Red) ...
```

可选参数只是 **函数调用** 的语法糖，实际上带有可选参数的函数的签名当中的参数仍然是全体参数。

<!--
#### 重载冲突

在判断判断一个同名函数的所有重载是否存在冲突时，编译器将会 **无视参数名称**，**无视是否可选参数**，仅依据参数的数据类型和顺序来判断。
-->

### 函数签名

函数的类型（函数的签名）可以作为一种数据类型。

`signature (type1, type2...) result type_name`

例如：

```js
signature (int x, int y) result int
signature<T, E> (T x, E y) result T
signature (T a, string s) type {T = int}
```

函数签名当中的参数可以省略名称，参数名称一般是为了帮助记忆参数的用途，但在编译时会直接被抛弃。

```js
// 省略了参数的名称
signature (int, int) result int
signature<T, E> (T, E) result T
signature (T, string) type {T = int}
```

### 空函数

`empty` 关键字用于在接口里定义无具体实现的函数。

`empty function name (...) result type_name`
`empty function name (...)`

空函数除了无函数主体，参数不能指定默认值。

在实现空函数时，函数的签名、参数的名称都必须一致。参数可以指定默认值。

### 永不返回函数

`function name (...) never_return {...}`

### 关联函数/方法

```js
implement DataType {
    function name (...) result ... {...}
}
```

### 接口

```js
interface HasName {
    empty function name (...) result ...
}
```

实现接口/应用接口

```js
implement HasName for DataType {
    ...
}
```

```js
实现 HasName 到 Data {
    ...
}
```

### 泛型

#### 泛型函数

`function name<T>(T left, T right) ...`

泛型参数具体化

`let a = name<type_name>(a, b)`

#### 泛型结构体

```js
struct Point<T> {
    T x
    T y
}
```

关联方法

```js
implement<T> Point<T> {
    function add(Self this) result T {
        this.x + this.y
    }
}
```

#### 泛型接口约束

```js
function max<T> (T left, T right) result T
    type {
        T = limit Ordered
    } {
    ...
}
```

一个类型多个约束，多个类型名称之间使用加号 `+` 拼接：

```js
function max<T> (T left, T right) result T
    type {
        T = limit Display + Ordered + Eq
    }
    {
        ...
}
```

### 关联类型

```js
interface Sequence associate ItemType {
    empty function first(Self s) result ItemType
}
```

关联类型的具体化：

```js
implement Sequence for User
    associate ItemType = int32 {
    ...
}
```

关联类型跟泛型的相同点是：允许一个接口的函数在具体实现时，可以有第二种由实现者指定的数据类型。而跟泛型不同的是，一个接口对于一个给定类型只能应用一次，因此这个接口的关联类型也只能指定一次。比如继续上一个例子：

```js
// 下面语句无法编译，因为上面已经关联过 int32
implement Sequence for User
    associate ItemType = int64 {

    }
```

关联类型的一些具体应用如 std::ops::Add，它的定义如下：

```js
interface Add<Rhs = Self> associate Output {
    empty function add(Self self, Rhs rhs) result Self::Output
}
```

注：对于上例中的 Output 为什么不使用泛型，因为给定 Lhs 和 Rhs 应该只有一种输出类型，而关联类型刚好能约束输出类型，所以应该使用关联类型表示输出类型。下面是用泛型表示输出类型的示例：

```js
interface Add<Rhs=Self, Output> {
    //...
}

implement<User, Users> Add<User, Users> for User {
    // 这表示 add(User, User) result Users
}

implement<User, Group> Add<User, Group> for User {
    // 这表示 add(User, User) result Group
    // 显然这会引起歧歧义，因为传入的类型一样，但输出的结果确有两种类型
}
```

关联类型一般应用在表示 “操作的输出” 这种场合，对于 From 和 Into 这类接口，就应该用泛型，而不能使用关联类型：

```js
// 标准的定义
interface From<T>  {
     empty function from(T value) -> Self;
}

// 错误的定义
interface From associate SourceType {
    empty function from(SourceType src) result Self
}
```

如果使用了上例当中的泛型来定义 From 接口，那么对于某个类型，比如 Int128，则只能应用一次 From 接口了，这显然不合适。

#### 多个关联类型

```js
interface Sequence associate (ItemType1, ItemType2) {
    empty function ItemType first(Self s)
}
```

具体化关联类型

```js
implement Sequence for DataType
    associate (ItemType1 = TypeName1, ItemType2 = TypeName2) {
    ...
}
```

#### 默认类型

```js
interface Convertable associate ItemType=TypeName {
    ...
}
```

多项默认类型

```js
interface Convertable associate (ItemType1=TypeName2, ItemType1=TypeName2) {
    ...
}
```

### 类型别名

`type 类型别名 = 源类型`

例如：

`type string = List<uint8>`
`type AddFn = signature (int x, int y) result int`

如果目标类型有泛型，可以声明部分泛型别名，或者全部泛型别名，例如：

`type OkOnly<T> = Result<T, std::Empty>`
`type MyResult<T, E> = Result<T, E>`

显然，如果目标类型有部分泛型既没有指定具体类型，也没有泛型别名，那么是语法错误的，例如：

`type MyResult = Result<T,std::Empty>`

编译器会以找不到名称为 `T` 的数据类型而报错。

<!-- ### 命名空间定义

```js
namespace tests {
    ...
}
``` -->

<!--
### 空类型/单值类型

`empty type 类型名`

空类型没有成员，无法实例化，它的值只有一个，且跟它本身同名，比如：

`empty type Empty`

它的类型是 `Empty`，其值也是 `Empty`。
-->

#### 命名空间路径

```js
foo::bar
```

### 导入标识符

```js
use std::List
use foo::{bar, baz}
use foo::{bar, sub::{one, two}}
```

### 标注

`#[name(...)]`

## 表达式

### let 表达式

标识符定义（兼赋值）表达式

`let left-hand-side = righ-hand-side`

例如：

`let int a = 10`

当左手边值是一个标识符时，可以省略数据类型，编译器会自动推导：

`let a = 10`

#### let 模式匹配

`let 表达式` 的 `左手边值`（`left-hand-side`） 既可以是一个变量，也可以是一个 `模式表达式`。`let 表达式` 也称为 `解构表达式`，例如：

```js
let List<int> [a,b] = ... // 列表解构
let User {id, name} = ... // 结构体解构
let User (id, name) = ... // 结构体解构
let User {id: user_id, name: user_name } = ...
let (int, int, string) (a,b,c) = ... // 元组解构
let Shape::Point {x, y} = ... // 枚举值解构
let Shape::Point (x, y) = ... // 枚举值解构
```

`模式表达式` 可以是 `标识符`、`元组`、`列表`、`映射表` 以及字面量等。

注意，解构表达式需要明确列出数据类型，另外当模式不匹配时，会引起运行时异常（而且无法捕捉和恢复）。

如果对解构后的部分数据不感兴趣，可以使用 _丢弃标识符_， 即下划线 `_` 来接收数据，比如：

`let (int, string) _, name = user001`

或者使用省略操作符丢弃其余部分：

`let List<int> [a,b, ...] = list001`

或者整个数据丢弃：

`let (int ,string) _ = user001`

当然整个数据都丢弃的话，虽然语法允许，但这条表达式是无意义的。

有关模式匹配表达式的详细说明，见语言参考文档。

#### let 表达式的返回值

`let 表达式` 返回右手边的值，而不管左手边是一个变量还是模式匹配表达式，例如：

`let User{id, name} = user001`

返回的是 `user001` 的值。

### do 表达式

`do {...}`

`do 表达式` 用于创建一个有自己作用域的 **代码块** (code block)。

代码块可包含一个或多个表达式，表达式会依次被求值，最后一个表达式的值将会作为代码块的值返回。

花括号里的表达式行末可以写逗号，也可以省略，比如：

```js
do {
    let i = 100
    let j = 200,     // 行末可以加上逗号
    writeLine(i+j)
}
```

> 同样使用花括号作为主体的 `多行格式的映射表`/`where`/`branch`/`match`/`do`/`join` 结构格式保持一致。

对于 `then`，`else`, `if`, `where` 等关键字，如果在其后面书写 `do 表达式`，则可以省略关键字 `do` 而直接写一对花括号，这种表达式块称为 `隠式 Do 表达式`。

代码块有自己的 **作用域** ，即，在代码块里定义的变量/对象在代码块结束时会自动清除，代码块的外部是无法访问代码块内部的变量/对象。

注意当一对花括号单独存在时，会被解析为 Map。

### if 表达式

`if ... then ... else ...`

一共三个从属表达式，其中 else 可以省略，其中 `if 子表达式`（即第一个从属表达式）要求返回 Boolean 类型的值，而 `then` 和 `else` 从属表达式既可以是单一一个表达式，也可以是表达式块 比如：

```js
if {let a = c * 2; a > b} then
    ...
else
    ...
```

if, then, else 关键字后面的表达式都允许换行写

if 表达式可以有返回值，可以定义一个变量或者其它形式来接收 if 表达式的返回值，需要注意的是，如有需要 if 表达式的返回值，需要保证每个分支的返回值的数据类型一致。

```js
let j = branch {
    case i>80: 'a',
    case i>60: 'b',
    default: 'c'
}
```

如果没有变量接收 if 表达式的返回值，则不要求各个分支的返回值数据类型，甚至不需要有返回值。但要注意如果 if 表达式位于函数的最后一句，其函数被定义为有返回值，则隐含这接收 if 表达式的返回值。

#### if where 从属表达式

`if 表达式` 的三个子表达式里面创建的标识符的作用域都仅仅局限在它们当前的子表达式（块）里，如果需要创建一个仅限当前 `if 表达式` 范围有效的标识符，可以使用 `if 表达式` 的 `where 从属表达式`：

```js
if a > 1 where let a = 2 then ...
if a > b where {
    let a = 2
    let b = 1 } then ...
```

注意 `where 子表达式` 先于 `if 子表达式（即 if 表达式的第一部分）` 执行，所以在 `where 子表达式` 里无法访问 `if 子表达式` 里的内容，但反过来是可以的。

`where` 也支持 `隠式 do 表达式`

### if let 表达式

`if let ... = ... then ... else ...`

`if let` 表达式并不是 `if 表达式` 和 `let 表达式` 两个表达式，而是固定的语法，是独立的一种表达式。

`if let` 表示：

- 当 `let` 关键字后面的两个表达式匹配时，执行 `then 子表达式`，且在 `then 子表达式` 里可以使用 `let` 关键字后面所创建的局部变量；
- 当 `let` 关键字后面的两个表达式不匹配时，并不会抛出运行时异常，而是执行 `else 子表达式`，显然因为匹配失败，所以在 `else 子表达式` 里不能访问 `let` 关键字后面所创建的局部变量。

示例：

```js
let User u = {id: 100, name: "foo"}
if let User{id, name} = u then
    write_line("id: {}, name: {}", id, name)
else
    write_line("not match")
```

上面的语句大致相当于：

```js
let User u = {id: 100, name: "foo"}
...
if result != std::Empty
    where {
        let result = match u {
            case User{id, name}:
                Ok((id, name))
            default:
                Err(std::Empty)
        };
    }
then
    let Ok((id, name)) = result
    write_line("id: {}, name: {}", id, name)
else
    write_line("not match")
```

### branch 表达式

```js
branch {
    case b==0: ...
    case b>a: ...
    default: ...
}
```

branch 表达式的每一个条件情况使用 `case 条件 : 结果` 这样的格式书写。case 和 case 之间使用 `换行符` 分隔

在 case 行末尾的逗号是可选的

branch 表达式可以视为多个 `if ... else ...` 表达式的紧凑写法。

> 同样使用花括号作为主体的 `多行格式的映射表`/`where`/`branch`/`match`/`do`/`join` 结构格式保持一致。

跟 if 表达式一样，branch 表达式也可以返回值，可以定义一个变量或者其它形式来接收 branch 表达式的返回值，需要注意的是，如有需要 branch 表达式的返回值，需要保证每个分支的返回值的数据类型一致。

```js
let j = branch {
    case i>80: 'a',
    case i>60: 'b',
    default: 'c'
}
```

如果没有变量接收 branch 的返回值，则不要求各个分支的返回值数据类型，甚至不需要有返回值。但要注意如果 branch 表达式位于函数的最后一句，其函数被定义为有返回值，则隐含这接收 branch 表达式的返回值。

#### branch where 从属表达式

branch 和 case 关键字后面都可以添加 where 从属表达式

branch 后面用于创建当前整个 branch 有效的作用域，比如

```js
branch where let a = 2 {
    ...
}
```

#### branch case where 从属表达式

case 后面可以加 where 从属表达式：

```js
branch {
    case b>a where let a = 1:
        a + 1
}
```

- case 后面用于创建当前 case 有效的作用域，比如
- 跟 `if 表达式` 的 `where 从属表达式` 的情况类似，case 的 `where 从属表达式` 也是先于 case 条件表达式执行。

### for 表达式

`for` 跟 `do` 一样，创建了一个带有作用域的 **代码块** ，跟 `do` 不同的是，`for` 可以定义入参数以及返回值的类型（所以 `for` 也可以视为子函数），而 `do` 不允许定义入参，且返回值的类型由其中的最后一个表达式决定。

`for` 的入参通过 `for` 关键字后面的 `let` 关键字定义：

`for let 变量 = 初始值 ...`
`for let 变量 = 初始值 {...}`

入参必须指定初始值。（入参在省略数据类型时，其数据类型是由初始值决定）

参数也可以是一个元组

`for let (a,b) = (0,1) {...}`

跟 `let` 表达式一样，参数也可以是一个模式匹配表达式，例如：

`for let User {name, ...} = user1 {...}`

注意，`右手边值` 不能是一个花括号型的结构体实例化表达式，因为会这个花括号会引起歧义，比如：

```js
for let i = User{id} {
    // 第一个花括号会被认为是循环体的开始符号，
    // 因此引起语法错误
}
```

改为如下即可：

```js
for let i = (User{id}) {
    ...
}
```

#### 省略入参的

for 表达式也可以省略参数列表，直接写成

`for {...}`

#### for let 多个参数

多个参数之间使用逗号分隔，注意每个参数都必须指定初始值：

```js
for let 变量1 = 初始值1, 变量2 = 初始值2 ...
```

#### for 的返回值

跟定义函数一样，`for` 表达式的返回值类型使用 `result` 关键字定义，比如：

`for let num=0 result int ...`

一般来说需要 `for` 返回一个值的情况比较少，所以一般的 `for` 表达式都是缺少 `result` 部分的，表示该表达式不返回值。

#### 使用 next 关键字实现循环

默认情况下，for 表达式的代码块里面的表达式会被依次求值，根据 for 表达式的返回值定义情况，最后一个表达式的值可能被返回，也可能被丢弃。然后 for 表达式结束。

不过，在 `for 表达式` 的语句块里面可以使用 `next` 关键字实现使用新的参数再次执行一次语句块，以实现循环结构：

```js
for let i = 0 {
    if i < 10 then {
        ...
        next i+1
    }
}
```

或者简写为：

```js
for let i = 0 if i < 10 then {
    ...
    next i+1
}
```

for 表达式定义了多少个参数，next 后面就要跟同样数量以及同样类型的值。`next` 是一个特殊的语句，在 `next` 语句之后的表达式不会被执行。

如果 for 表达式没有定义参数，则 `next` 后面也不能添加任何数值，比如：

```js
for {
    if getValue() == 0 then {
        next
    }
}
```

#### for 的 `break` 语句

当 for 表达式定义了返回值数据类型时，表达式块里面最后一条非 `next` 表达式的值将会作为返回值。

比如：

```js
let i = for let index = 0 result int{
    let value = getValue(index)
    if value == 0 then {
        next index+1
    }else {
        index
    }
}
```

也可以使用 `break` 关键字提前返回值（跟函数的 `return` 关键字相似），比如：

```js
let sum = for let i = 10 result int {
    ...
    if (i)
    break 123
}

```

### for let ... in ... 表达式

用于逐个获取集合里的元素。

`for let 变量 in 集合 ...`
`for let 变量 in  集合{...}`
`现有 让 变量 取自 集合 {...}` // for...in 非一对一关键字翻译

```js
for let i in [1,2,3] {
    i*2
}
```

<!-- each 返回 List。-->

注意，关键字 `in` 后面不能跟一个花括号型的结构体实例化表达式，因为这个花括号会引起歧义，比如：

```js
let id = 123
let numbers = [1,2,3]
for let i in User{id, numbers}.numbers {
    // 第一个花括号会被认为是循环体的开始符号，
    // 因此引起语法错误
}
```

如果的确需要一个带花括号的结构体实例化表达式，可以使用括号包围起来：

```js
for let i in (User{id, numbers}.numbers) {
    ...
}
```

列表的 `map()` 方法提供了跟 for...in 一样的功能：

```js
[1,2,3].each(fn i {...})
```

### match 表达式 （模式匹配）

```js
match v {
    case a: expression
    case b: {...}
    default: ...
}
```

match 表达式的每一个匹配情况使用 `case 条件 : 结果` 这样的格式书写。case 和 case 之间使用 `换行符` 分隔。

case 行末尾的逗号是可选的。

> 同样使用花括号作为主体的 `多行格式的映射表`/`where`/`branch`/`match`/`do`/`join` 结构格式保持一致。

`case` 后面是一个由：`变量` + `模式表达式` + `从属表达式` 共 3 个部分组成的结构，这 3 个部分都是可选的，但 `变量` 和 `模式表达式` 这 2 个部分至少必须有其中的一个部分。

注意，关键字 `match` 后面不能跟一个花括号型的结构体实例化表达式，因为这个花括号会引起歧义，比如：

```js
let id = 123
match User {id} {
    // 第一个花括号会被认为是模式匹配主体的开始符号，
    // 因此引起语法错误
}
```

改为如下即可：

```js
match (User {id}) {
    ...
}
```

#### match where 从属表达式

`match` 后面可以加上 `where 从属表达式`

```js
match v where ... {
    ...
}
```

`match` 关键字后面的 `where 从属表达式` 的作用域覆盖整个 `match 表达式`，包括每一个 `case`。

#### match case 变量部分

在 case 后面（在模式表达式之前）可以添加一个标识符然后接着符号 `@`，用于保留被匹配的数据

```js
match some_express {
    case u1 @ User{name}: ... // 这里的 u1 的值为 some_express 表达式返回的值
}
```

#### match case 从属表达式

##### where 从属表达式

`case` 后面也可以加上 `where 从属表达式`

```js
match v {
    case a where let b = a + 1: // 此时 a 是可以访问的，因为仅当 a 匹配了才进入当前分支
        b
}
```

- `case` 后面的 `where 从属表达式` 的作用域仅覆盖当前 `case`。
- 跟 `if 表达式` 的 `where 从属表达式` 的情况类似，case 的 `where 从属表达式` 也是先于当前 case 条件表达式以及当前 case 的 **所有其他从属表达式** 执行。不同的是， 在 where 里可以访问已匹配的变量。比如上例当中的 a。
- case 的 where 丛属表示式一般搭配 only 从属表达式使用。

##### only 从属表达式

也叫守护表达式

```js
match v {
    case a only b > 10 where let b = sqrt(a):
        b + 1
}
```

`only 从属表达式` 可以是单一一个表达式，也可以是一个表达式块，只需返回值为 `Boolean 类型` 即可。`only 从属表达式` 在所有 case 从属表达式当中最后一个被执行（最后一道防线，相应地，where 从属表达式是第一个被执行的）。

注：在有些编程语言里，守护表达式也用 if 关键字，但为了便于区分，这里使用一个新的关键字 “only”。

<!--
#### 模式表达式

`let 表达式` 左手边值也是 `模式表达式`，模式表达式可以是：

- 元组
- 列表
- 映射表
- 结构体（包括元组型和映射表型）的实例化表达式
- 字面量

但 `match case` 的模式表达式还可以是： -->

##### in 模式表达式

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

关键字 `in` 后面可以是一个 `Range`、一个 `List` 对象，只要是一个拥有 `Exist` 接口的对象都可以。

##### into 模式表达式

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

##### regular 模式表达式

case 后面可以添加 regular 关键字

regular 后面两个表达式，一个表达式正则表达式的字符串字面量，另一个是标识符列表

```js
let s = "foo@domain"

match s {
    case regular ~/^(.+)@(.+)$/ [_, Some(name), Some(domain)]:
        writeLineFormat("It's Email: {}@{}", name, domain)
    case regular ~/^(\\w+):\/\/(.+)$/ [Some(phone), Some(countryCode), Some(number)]:
        writeLineFormat("It's phone number: {}", phone)
    default:
        writeLine("Not detected")
}
```

##### template 模式表达式

case 后面可以加 template 关键字，关键字后面跟着一个字符串字面量，字符串里可以添加占位符 `{...}`，占位符需要写上该占位符的名称，以及正则表达式。如果省略正则表达式，默认为 `\w+`

```js
match s {
    case template "/user/{userName:\w+}":
        writeLineFormat("Get user {}", userName)
    case template "/user/{userName:\w+}/post/{postId:\d+}":
        writeLineFormat("Get post {}", postId)
}
```

在编译器内部，template 会被转换为 regular 模式

#### 嵌套匹配模式

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

### 模式函数

添加了关键字 `pattern` 的函数，其参数可以使用模式匹配表达式（包括 match 表达式 case 关键字后面的各种从属表达式）。模式函数必须同名、同参数、同返回类型（指：函数名称相同，参数个数、参数出现的顺序和数据类型都必须相同，返回数据类型也必须相同，仅每个参数的模式表达式不同）。

```js
pattern function test (string s @ parse Email email, string s @ parse Phone phone) {
    ...
}
```

同名的模式匹配函数会被编译器转换为 branch 结构。编译器会把它们全部组合为一个函数。

跟 `match 表达式` 的 `case` 不同，模式函数的参数需要指出参数的数据类型，除非出现了 `regular`、`template` 这两种从属表达式。因为它们要求被匹配的数据只能是 `string` 类型。

```js
pattern function test (
    int i @ in [1..10],
    Point (x, y) only x + y > c
        where let c = x - y),
    User {id, name} only id > 100
    {
    ...
}
```

另外，模式函数本身也支持函数范围的 `only 从属表达式`（注意，模式函数的 `only 从属表达式` 生效在所有参数解析完之后），它作为模式匹配函数最后的一道防线。

```js
pattern function test(int x, int y)
    only x > y {
        ...
}
```

> 模式函数的参数不能是可选参数。

### fn 表达式（匿名函数/子函数）

`fn (type_name param_name) result type_name = ...`
`fn (type_name param_name) result type_name {...}`

只要能在上下文环境中推导出来，匿名函数的参数类型可以省略，返回值数据类型也可以省略：

`fn (param1, param2) = ...`

无参数时的形式：

`fn () = ...`

当参数只有一个，且省略了参数数据类型和返回值的数据类型时，匿名函数可以进一步简化为：FunctionDeclaration

`fn param_name = ...`

比如：

```js
let s = [1,2,3]
    .map(fn x = x.to_string())
    .join("")
```

跟普通函数一样，匿名函数也支持 `type` 从属表达式。

跟普通函数不同的是：

- 匿名函数没有函数名称；
- 参数和返回值的类型可以省略，由上下文推导出来；
- 不支持泛型
- 不支持参数默认值。

### 函数调用

#### 普通形式

```js
name(value1, value2, value3)
name(name1=value1, name2=value2, name3=value3)
```

- 被调用者必须是一个标识符、一个对象的成员（属性值或索引值）、或者一个匿名函数；
  foo(...)
  foo.bar(...)
  foo[0](...)
  (fn x=x+1)(...)

- 被调用者也可以是一个用括号包围起来的表达式或者表达式块，只要是返回函数即可；
  (foo & bar)(...)

- 允许连续调用。
  foo(...)(...)    // 连续调用

#### 中置调用

`a :fn_name: b`

#### 前置调用 （考虑取消）

`!fn_name (data1, arg1, arg2)`

等同于 :

`data1.fn_name(arg1, arg2)`

或者

`name::path::fu_name(data1, arg1, arg2)`

#### 匿名函数调用示例

```js
users
    .map(fn (x) = x*2)
    .filter(fn x = x>3)
```

## 符号（按优先级列举）

### 运算符号

- `=` 赋值语句
- `|` 管道

二元运算符

- `:name:` 函数中置调用
- `||` 逻辑或运算
- `&&` 逻辑与运算
- `== !=` 相等比较
- `> >= < <=` 大小比较
- `->` 链式调用，类似 `:result_and:`
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

- `!` 函数前置调用，符号必须位于标识符之前（考虑取消）
- `(...)` 分组
- `(... ,)` 元组
<!-- - `[...], #[...], ![...]` 列表、数组、矩阵 -->
- `[...]` 列表
- `{...}` 映射表
- `[from..to] [one,two..to]` 范围数列

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
function add(int a, int b) {...}

let inc_ten = add(10, _)
inc_ten(5) // return 15
```

`_name` 表示命名标识符占位符，在构建部分调用函数时，可以重新为新参数命名：

```js
function draw(Point point, int width, Color color) {...}

let draw_thin_line = draw(_p, 1, _c)
draw_thin_line({10,20}, Color::Red)
draw_thin_line(p: {10,20}, c: Color::Red)

```

## 宏

宏（macro）用于在编译前自动生成部分源代码，以减少手动录入代码的量。宏会在编译前 “展开” 为普通代码，不过这个过程是发生在编译器内部的。内置的几个宏有 join，println 等。

注：宏实际上就是 “自动代码生成” 的指令，它通过解析语法树来生成相应的代码，除了内置宏，也可以用户自定义。

`join! {...}`

join 宏一般用于构建字符串。表达式里允许一个或多个表达式，每个表达式的值都会被转为字符串（通过执行值数据的 `.toString()` 方法），然后（无分隔符）拼接起来。

示例：

```js
join {
    "hello "
    "world!"
    "foo "
    "bar."
}
```

另一个示例：

```js
join! {
`<section id="user">`
    `<h1>User List</h1>`
    for let user in users {
        `<div id="{{user.id}}">{{user.name}}</a>`
    }
`</section>`
}
```

注意 join 返回的是一个列表（List），列表的 `.toString()` 方法将会对其中所有元素求字符串值，然后将所有字符串连接起来（无分隔符）。

在花括号里的表达式，行末可以写逗号，也可以省略，比如：

```js
join! {
    "hello "
    "world!",     // 行末可以加上逗号
    date::now()
}
```

> 同样使用花括号作为主体的 `多行格式的映射表`/`where`/`branch`/`match`/`do`/`join!` 结构格式保持一致。

<!-- #### 将拼接后的字符串传递给指定函数

`join to some_function_name {...}`

`some_function_name` 可以是一个函数的名称，也可以是一个值为函数的表达式，函数必须只有一个 string 类型的参数。

```js
join to format(_, "date") {
    ...
}
``` -->
