# 变量

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [变量](#变量)
  - [变量的声明和赋值](#变量的声明和赋值)
    - [数据类型](#数据类型)
      - [类型推导](#类型推导)
      - [列表（List）](#列表list)
      - [映射表（Map）](#映射表map)
      - [元组](#元组)
      - [函数](#函数)
        - [类型简写](#类型简写)
    - [定义数据类型别名](#定义数据类型别名)
  - [值的不可变性](#值的不可变性)
  - [变量的不可变性](#变量的不可变性)
  - [变量的可变性（::不支持）](#变量的可变性不支持)
    - [变量的作用域](#变量的作用域)
    - [全局变量（::不支持）](#全局变量不支持)
  - [常量](#常量)
    - [局部常量](#局部常量)
  - [枚举](#枚举)

<!-- /code_chunk_output -->

## 变量的声明和赋值

变量的声明和赋值需要同时进行，语法如下：

* `让 数据类型 变量名称 = 值`
* `let DataType name = value`

示例：

* `让 整数 a = 123`
* `let Int a = 123`

在 XiaoXuan 语言里，基本类型的数据是值，结构体、联合体也是值，一个函数也可以作为一个值，所以它们都可以赋值给一个变量。

### 数据类型

#### 类型推导

XiaoXuan 有类型推导机制，当值的类型可以确定的情况下，可以省略变量的数据类型声明，示例：

* `让 a = 123`
* `let a = 123`

变量 `a` 的数据类型会自动推导为 `整数`（`Int`）。

类型推导还具有传导性，函数的嵌套调用、函数的连续调用都会尽可能地推导出正确的数据类型。

一般情况下，对于变量声明及赋值语句：

* 如果是字面值赋值，**建议省略** 变量的数据类型声明，因为数据类型显而易见，省略数据类型声明不妨碍阅读理解且会更简洁；
* 如果是接收某个函数的返回值，**不建议省略** 变量的数据类型声明，因为调用其他模块的函数，其返回值的数据类型不明显，加上数据类型声明能让代码更清晰。
* 如果是接收范型的函数的返回值，当范型作用在返回值类型时，可以省略变量的数据类型声明，当范型是作用在参数时，如果是实例化一个联合体，则可以省略，其他情况则不建议省略。比如 `Option<T>`，语句 `let a = Some(123)` 可省略类型。

#### 列表（List）

列表字面量的数据类型由第一个元素决定，比如：

* `让 a = [1, 2, 3]`
* `let a = [1, 2, 3]`

列表的数据类型为 `列表<整数>`（`List<Int>`）。

* `让 b = ["你好", "世界"]`
* `let b = ["Hello", "World"]`

列表的数据类型为 `列表<字符串>`（`List<String>`）。

因为空列表无法进行类型推导，所以要赋值一个空列表给变量，则需要声明变量的数据类型：

* `让 列表<字符串> b = []`
* `let List<String> b = []`

当然这种情况下一般使用列表的构造函数来实例化一个空列表。

* `让 c = 列表<字符串> ()`
* `let c = List<String> ()`

#### 映射表（Map）

映射表的数据类型由第一个映射对的数据类型决定，比如：

* `让 a = {姓: "张", 名: "三"}`
* `let a = {firstName: "San", lastName: "Zhang"}`

因为第一对映射对是：`姓: "张"`，映射表的数据类型为 `映射表<字符串, 字符串>`（`Map<String, String>`）。

注意当映射表的字面量当中的键名是字符串时，是可以省略包围字符串的双引号的，所以上例实际上是下面语句的简写：

* `让 a = {"姓": "张", "名": "三"}`
* `let a = {"firstName": "San", "lastName": "Zhang"}`

#### 元组

元组可视为一个匿名成员的结构体，元组的数据类型由其中的各个元素决定，即，元组的数据类型是其各个成员的顺序和数据类型的组合。比如：

* `让 a = (123, "你好", 真)`
* `let a = (123, "Hello", true)`

则这个元组的数据类型是 `(整数, 字符串, 逻辑)`（`(Int, String, Boolean)`）

元组一般作为函数的参数列表，也可以作为函数的返回值。

作为函数的参数时，可以为元组的字面量的成员添加名称，运行环境会根据名称来找到对应的参数然后传值。示例：

* `让 p = (id = 123, name = "张三")`
* `let p = (id = 123, name = "foo")`

但语法不支持根据名称获取成员值，简单来说，元组成员的名称是只写的。

当一个函数的返回值不止一个数据，但又不想单独创建一个结构体来存储这个返回值时，可以简单地使用元组 "封装" 这些值，然后一次返回给函数的调用者。

有些函数可能没有任何具有意义返回值，所以专门有一个叫做 `单元`（`Unit`）的联合体（数据类型），其值有且只有一个 `空值`（`void`）。XiaoXuan 规定空元组 "()" 的数据类型为 `单元`，其值等于 `空值`。

#### 函数

函数的签名就是该函数的数据类型，比如下面的函数：

* `函数 整数 增加(整数 number, 整数 amount) = number + amount`
* `function Int increase(Int number, Int amount) = number + amount`

它的签名是：

* `整数 <= (整数, 整数)`
* `Int <= (Int, Int)`

其中 `<=` 符号用于分隔函数返回值的数据类型以及函数的参数列表，也表示这个表达式是一个函数签名。

在字面上，函数的签名如同把函数的名称、参数的名称、函数主体通通全部移除之后的骨架，也就是说，函数签名只包含：

* 函数返回值的数据类型
* 各参数的数据类型的列表

有时为了便于阅读理解，也可以保留函数的签名的参数名称，比如:

* `整数 <= (整数 number, 整数 amount)`
* `Int <= (Int number, Int amount)`

对于范型（即类型参数化）函数，参数的数据类型就是范型参数，比如：

* `函数 甲型 增加(甲型 source, 乙型 amount) =  source + amount`
* `function T increase(T source, E amount) =  source + amount`

它的签名是：

* `甲型 <= (甲型, 乙型)`
* `T <= (T, E)`

如果一个函数的参数的数据类型是函数，则该函数的定义语句可能会很长以至于影响阅读，比如：

```js
函数 返回值的数据类型 函数名称 (整数 <= (整数 a, 整数 b) 参数1, ..., 字符串 <= (字符串 s, 字符 c) 参数N)
    ...
以上
```

```js
function DataType function_name (Int <= (Int a, Int b) param1, ..., String <= (String s, Char c) paramN)
    ...
end
```

##### 类型简写

XiaoXuan 语法支持 `其中`（`where`） 关键字，以允许在函数的主体之前将函数的签名定义成一个单独的名称，语法如下：

```js
函数 返回值的数据类型 函数名称 (签名名称1 参数1, ..., 签名名称N 参数N) 其中
    签名名称1 = 整数 <= (整数 a, 整数 b),
    签名名称N = 字符串 <= (字符串 s, 字符 c)
    ...
以上
```

```js
function DataType function_name (type1 param1, ..., typeN paramN) where
    type1 = Int <= (Int a, Int b),
    typeN = String <= (String s, Char c)
    ...
end
```

语法当中的 `其中`（`where`） 关键字接着一系列参数类型的声明，多个类型声明之间使用逗号分隔，最后一个类型声明之后则是函数的主体开始。

### 定义数据类型别名

可以为名字较长的数据类型（如某个组合的元组，或者某个函数签名）定义一个别名，使用别名可以让代码更简洁清晰，定义数据类型别名的语法：

* `类型 名称 = 数据类型`
* `type Name = DataType`

示例：

```js
类型 字符串 = 列表<字符>
类型 空型 = 结果<单元, 错误>
类型 排队令牌 = (整数, 字符串)
类型 整数过滤函数 = 逻辑 <= (整数)
类型 过滤函数<T> = 逻辑 <= (T)
```

```js
type String = List<Char>
type Void = Result<Unit, Error>
type Token = (Int, String)
type IntFilterFunc = Boolean <= (Int)
type FilterFunc<T> = Boolean <= (T)
```

使用类型别名可以缩短函数的定义语句，比如下面几个函数的定义的结果是一样的。

```js
// 直列式

函数 列表<整数> 筛选合格者(列表<整数> items, 逻辑 <= (整数) f)
    ...
以上

// 使用 `其中` 关键字

函数 列表<整数> 筛选合格者(列表<整数> items, 整数过滤函数 f) 其中
    整数过滤函数 = 逻辑 <= (整数)
    ...
以上

// 使用 `类型` 定义别名

类型 整数过滤函数 = 逻辑 <= (整数)
函数 列表<整数> 筛选合格者(列表<整数> items, 整数过滤函数 f)
    ...
以上
```

```js
// flat list

function List<Int> passFilter(List<Int> items, Boolean <= (Int) f)
    ...
end

// using 'where` keyword

function List<Int> passFilter(List<Int> items, IntFilterFunc f) where
    IntFilterFunc = Boolean <= (Int)
    ...
end

// using 'type' define alias name

type IntFilterFunc = Boolean <= (Int)
function List<Int> passFilter(List<Int> items, IntFilterFunc f)
    ...
end
```

## 值的不可变性

XiaoXuan 的值（包括各种集合、结构体、联合体等）也是不可变的。比如：

* 对于一个列表，直接修改它的元素是不允许的，增加或者删除元素都会返回一个新的列表；
* 对于一个结构体，无法修改它的成员的值，修改某个成员的值会返回一个新的结构体。

值得不可变性使得数据可以放心传递给多线程，不会因为某个线程的修改而导致数据竞争。是 XiaoXuan 并行和并发工作模式的基础。

## 变量的不可变性

变量定义并赋值之后，便无法再次赋值。也就是说，除了变量指向的值是不可变的，变量的 "指向" 本身也是不可变的。

比如下面的语句会引起运行时异常：

```js
让 a = 123
a = 456  // 错误
```

```js
let a = 123
a = 456  // Error
```

因为变量无法重新赋值，所以对于我们所熟悉的（在可变变量的语言里）常用数据处理方法，在 XiaoXuan 里需要稍微转换一下。

比如有一个程序：让用户多次输入数字，然后计算数字的总和，直到用户输入 "Q" 为止。下面的是 JavaScript 版本：

```JavaScript
let sum = 0;
while(true) {
    let s = prompt('Enter a number:');
    if (s === 'Q') break;

    let i = parseInt(s);
    sum = sum + i;
}
console.log(sum)
```

下面是 XiaoXuan 版本：

```js
function Int add(Int acc)
  let s = readLine()?
  if s == "Q" then return acc

  let i = parse<Int>(s)
  let c = acc + i

  // 以当前的累加值作为参数调用自己以
  // 再执行一遍当前这个过程
  add (c)
end

let sum = add (0)
writeLine(sum)
```

XiaoXuan 语言会对函数的尾调用进行优化，只要函数最后执行的表达式是调用函数自己本身，则不会创建新的调用栈，也就是说不会堆栈溢出。

XiaoXuan 也有自己的条件循环语句：`设有 让...`（`for let...`）语句，其实质是通过创建一个匿名函数及递归调用来实现的，这样可以省去自己手动创建函数。示例：

```js
let sum =
    for let Int acc = 0
        let s = readLine()?
        if s == "Q" then return acc

        let i = parse<Int>(s)
        let c = acc + i
        loop c
    end

writeLine(c)
```

`for let...` 语句实际上是创建了一个匿名函数，然后通过 loop 语句调用自己，以实现 "循环" 的效果。详细请见 [流程控制](control-flow.md)。

## 变量的可变性（::不支持）

某些 XiaoXuan 的衍生版本（语言）可能会支持可变变量，但其可变原理跟一般支持变量可变的语言仍不太相同。

比如，现有一个指向字符串的变量，在被重新赋值之后，该变量原先指向的那份（在内存堆中）字符串数据仍然被保留着，只是变量指向了一份新的字符串数据。当程序再次读取这个变量时，获取的是最新版本的数据。至于旧的版本，如果之前已经被其他程序读取且仍在使用中（比如已经传递给某一个线程正在处理），则这部分程序看到的仍然是旧版本的数据。如果旧版本没有其他程序引用，则它将被垃圾回收。

可见处理可变变量的方式总是 "只增" 新版本，然后让变量指向新版本，而不是直接修改（或覆盖，或抹除）原先的数据。这种处理方式类似某些版本控制工具（比如 Git）的工作方式，即默认情况下提交（commit，更新）仓库实际上是增加新版本，所有历史版本都会保留着。

### 变量的作用域

每个变量都有其作用域，比如：

* 在模块定义的变量，其作用域是其所在的模块（同一个模块可以分布在不同的源代码文件里）；
* 在函数里定义的变量，其作用域在其所在的函数里；
* 在语句块（包括条件语句、循环语句、匿名函数主体等）定义的变量，其作用域仅限其所在的语句块里。

示例：

```js
module apple

define val n = 11 // 定义模块级的变量 n

define func Result<Unit, Error> a ()
    writeLine(n) // 输出模块级的 n = 11
    let n = 22   // 定义函数级的变量 n，覆盖了模块级的变量 n
    writeLine(n) // 输出函数级的 n = 22

    for d in [1..10] // 变量 d 的作用域仅限当前 foreach 语句块之内
        writeLine(n) // 输出函数级的 n = 22
        let n = 33   // 定义语句块级的变量 n，覆盖了函数级的变量 n
        writeLine(n) // 输出语句块级的变量 n = 33

        if d>0 then
            writeLine(n) // 输出语句块级的变量 n = 33
            let n = 44   // 定义较内层的语句块级的变量 n，覆盖了较外层的变量 n
            writeLine(n) // 输出较内层的语句块级的变量 n = 44
        end

        writeLine(n) // 输出语句块级的变量 n = 33
    end

    writeLine(n) // 输出函数级的 n = 22
end

define func Result<Unit, Error> b ()
    writeLine(n) // 输出模块级的 n = 11
end
```

注意在同一层作用域里定义同名的变量是不允许的，比如下面的代码会引起运行时异常：

```js
函数 空型 测试()
    让 n = 123
    如果 n > 100 那么
      ...
    以上

    让 n = 456 // 这句会引起运行时异常
以上
```

```js
function Void test()
    let n = 123
    if n > 100 then
      ...
    end

    let n = 456 // Runtime error
end
```

### 全局变量（::不支持）

全局变量其实就是作用域为模块级的普通变量，即在模块范围里（而不是在某个函数、类型实现、模块实现里）定义的变量。全局变量使用关键字 `定义` 来定义，其语法除了关键字不同，其余跟 `让` 语句一样。示例：

```js
模块 foo.bar

定义 整数 n = 123

@测试
函数 空型 第一个测试 ()
    输出行 (n) // 这里输出全局变量 n 的值
以上
```

```js
module foo.bar

define Int n = 123

@test
function Void firstTest ()
    writeLine (n) // output the value of global variable n
end
```

在模块范围定义的变量能够被其他模块所读取，只要被导入即可，也就是说它能被程序的任何地方所读取，所以也可以称为 "全局变量"。

示例：

```js
模块 hello.world

导入 foo.bar

@测试
函数 空型 第二个测试 ()
    输出行 (bar.n) // 这里输出 'foo.bar' 模块的变量 n 的值
以上
```

```js
module hello.world

import foo.bar

@test
function Void secondTest ()
    writeLine (bar.n) // output the value of global variable n which exists in the module 'foo.bar'
end
```

全局变量的赋值语句（即右值，一个字面量或者一个表达式）会在模块被加载时运行。准确来说，加载应用程序时，会先加载所有模块所有定义性质的语句，比如函数、结构体、共性、接口、组件实现等等的定义，然后才执行全局变量的赋值语句。

## 常量

因为 XiaoXuan 的变量的值不可变性，所以常量跟变量非常相近，不同的地方在于：

* 常量即使在某些支持变量可变的衍生版本里，也是不能被重新赋值；
* 常量的名称可以用在模式匹配里当作字面量使用，而变量名称在模式匹配里会被当成新的变量。

定义常量的语法是：

```js
常量 数据类型 常量组名称
   成员名称1 = 值1
   成员名称2 = 值2
   ...
   成员名称N = 值N
以上
```

XiaoXuan 规定常量必须一组一组地定义，不能单独地使用一个名称定义一个常量值，访问的时候使用 `组名.成员名` 的格式。

示例：

```js
模块 http.client

常量 整数 ResponseCode
    Ok = 200
    Moved = 301
    Found = 302
    NotModified = 304
    BadRequest = 400
    Forbidden = 403
    NotFound = 404
    ServerError = 500
    ServiceUnavailable = 503
以上

函数 结果<单元> 第一个测试 ()
    输出行 (ResponseCode.Ok) // 输出常量的实际值 "200"
    输出行 (ResponseCode.NotFound) // 输出 "404"
以上
```

```js
module http.client

const Int ResponseCode
    Ok = 200
    Moved = 301
    Found = 302
    NotModified = 304
    BadRequest = 400
    Forbidden = 403
    NotFound = 404
    ServerError = 500
    ServiceUnavailable = 503
end

function Result<Unit, Error> firstTest ()
    writeLine (ResponseCode.Ok) // output the actual value "200"
    writeLine (ResponseCode.NotFound) // output "404"
end
```

在模块里定义的常量可以在程序的任何地方所读取，所以也称为 "全局常量"，示例：

```js
模块 main

函数 结果<单元, 错误> 第二个测试 ()
    导入 http.client.ResponseCode // 导入语句可以写在任何地方

    输出行 (ResponseCode.Ok) // 输出 "200"
    输出行 (ResponseCode.NotFound) // 输出 "404"
以上
```

```js
module main

function Result<Unit, Error> secondTest ()
    import http.client.ResponseCode // Import statements can be written anywhere

    writeLine (ResponseCode.Ok) // output "200"
    writeLine (ResponseCode.NotFound) // output "404"
end
```

注意：

* 导入全局常量时必须整组导入，不能只导入单独一个值，比如 `import http.client.ResponseCode` 是对的，但 `import http.client.ResponseCode.Ok` 是错误的。
* 常量组不是一种数据类型，不能作为变量或者参数的数据类型，它仅仅是名称的一部分而已。
* 常量的值只能是基本的数据类型，或者结构体、联合体使用默认构造函数创建的实例，不能是一个表达式，即常量的右值不能是一表达式，比如一个函数的返回值。

### 局部常量

有时可能只想在某一个函数里要一个严格只读的值，而不是想定义全局常量，比如在模式匹配里，想定义一个临时的常量（即在地方地方用不着的常量）使用，对于在函数范围内定义的常量，可以不需要定义为一个组，即支持使用一个名称定义一个常量，语法：

* `常量 数据类型 常量名称 = 值`
* `const DataType ConstantName = value`

示例：

```js
function test()
    const Good = 100
    match s
        case Good: writeLine("Good")
        case _: writeLine("Other")
    end
end
```

注意在模式匹配中，不能使用变量来代替常量：

```js
function test()
    let Good = 100
    match s
        case Good: writeLine("Good") // 这里的 Good 将会是一个新的变量，其值等于变量 s
        case _: writeLine("Other") // 这行永远不会被执行
    end
end
```

## 枚举

枚举跟全局常量相类似，不过枚举有以下几个特点：

* 枚举的成员数量固定；
* 枚举的成员的值无需指定；
* 枚举是一种数据类型；
* 枚举的值无法跟其他数据类型直接转换。

示例：

```js
枚举 原色
    红
    绿
    蓝
以上

让 a = 原色.蓝
```

```js
enum PrimaryColor
    Red
    Green
    Blue
end

let a = PrimaryColor.Blue
```

因为枚举是数据类型，所以枚举可以用在函数的参数上，用于某个参数只能从有限的几种值当中取其一的这种场合。示例：

```js
函数 设置背景色 (原色 c)
    输出行 (c)
以上

// 调用函数
设置背景色 (原色.红)
```

```js
function setBackgroundColor (PrimaryColor c)
    writeLine (c)
end

// call function
setBackgroundColor (PrimaryColor.Red)
```

注意枚举值**不能**与其他数据类型的值相互转换，比如无法将整数转为上例中的 `原色` 的值（也无法将枚举的值转成整数）。

例如下面的语句是错误的：

```js
设置背景色 (1)
```

```js
setBackgroundColor (1)
```
