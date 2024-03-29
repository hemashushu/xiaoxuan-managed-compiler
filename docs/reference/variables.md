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
        - [类型简写（考虑移动到函数一章）](#类型简写考虑移动到函数一章)
    - [定义数据类型别名](#定义数据类型别名)
    - [字面两类型的自动（隐式）转换](#字面两类型的自动隐式转换)
  - [值的不可变性](#值的不可变性)
  - [变量的不可变性](#变量的不可变性)
  - [变量的可变性（::不支持）](#变量的可变性不支持)
    - [变量的作用域](#变量的作用域)
    - [全局变量（::不支持）](#全局变量不支持)
  - [常量](#常量)
  - [枚举](#枚举)
  - [值的复制](#值的复制)

<!-- /code_chunk_output -->

## 变量的声明和赋值

变量的声明和赋值需要同时进行，语法如下：

* `让 数据类型 变量名称 = 值`
* `let DataType name = value`

示例：

* `让 整数 a = 123`
* `let Int a = 123`

在 XiaoXuan 语言里，基本类型的数据是值，结构体、联合体也是值，一个函数也可以作为一个值，所以它们都可以赋值给一个变量。

变量名可以是大小写字母、中文（严格来说代码点大于 u{00A0} 的 Unicode 字符均可）、数字和下划线的组合，但第一个字不能是数字，变量名区分大小写。

::TODO

`assign` 赋值语句：
`assign<Int>(a, 123)`

`赋予变量甲,真`
`赋予<逻辑>变量甲,真`
`赋予变量乙,一二三`
`赋予(整数)变量乙,一二三`

```js
赋予<逻辑>变量甲,真
如果变量甲:等于:真那么书写行 "文本一" 否则书写行 "文本二"
若甲:同:真则书 "文一" 否则书 "文二"
```

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

* `整数 <- (整数, 整数)`
* `Int <- (Int, Int)`

其中 `<-` 符号用于分隔函数返回值的数据类型以及函数的参数列表，也表示这个表达式是一个函数签名。

在字面上，函数的签名如同把函数的名称、参数的名称、函数主体通通全部移除之后的骨架，也就是说，函数签名只包含：

* 函数返回值的数据类型
* 各参数的数据类型的列表

有时为了便于阅读理解，也可以保留函数的签名的参数名称，比如:

* `整数 <- (整数 number, 整数 amount)`
* `Int <- (Int number, Int amount)`

对于范型（即类型参数化）函数，参数的数据类型就是范型参数，比如：

* `函数 甲型 增加(甲型 source, 乙型 amount) =  source + amount`
* `function T increase(T source, E amount) =  source + amount`

它的签名是：

* `甲型 <- (甲型, 乙型)`
* `T <- (T, E)`

如果一个函数的参数的数据类型是函数，则该函数的定义语句可能会很长以至于影响阅读，比如：

```js
函数 返回值的数据类型 函数名称 (整数 <- (整数 a, 整数 b) 参数1, ..., 字符串 <- (字符串 s, 字符 c) 参数N)
    ...
以上
```

```js
function DataType function_name (Int <- (Int a, Int b) param1, ..., String <- (String s, Char c) paramN)
    ...
end
```

#### 结构体

::TODO

```js
struct User
    Int number
    String name
end

List<User> users = [User::new(1,"foo"), User::new(2, "bar")]

// 可以省略列表内元素的类型，由变量声明的类型自动推导

List<User> users = [{1, "foo"}, {2, "bar"}]
```

##### 类型简写（考虑移动到函数一章）

XiaoXuan 语法支持 `类型`（`type`） 关键字，以允许在函数的主体之前将函数的签名定义成一个单独的名称，语法如下：

```js
函数 返回值的数据类型 函数名称 (签名名称1 参数1, ..., 签名名称N 参数N) 类型
    签名名称1 = 整数 <- (整数 a, 整数 b),
    签名名称N = 字符串 <- (字符串 s, 字符 c)
    ...
以上
```

```js
function DataType function_name (type1 param1, ..., typeN paramN) type
    type1 = Int <- (Int a, Int b),
    typeN = String <- (String s, Char c)
    ...
end
```

语法当中的 `类型`（`type`） 关键字接着一系列参数类型的声明，多个类型声明之间使用**逗号分隔**，最后一个类型声明之后则是函数的主体开始。

> 如果有多个类型声明，类型声明之间的逗号**不能省略**，因为这是表示类型声明部分，当最后一个声明后面没有逗号时，语法解析器会认为是函数主体的开始。

### 定义数据类型别名

可以为名字较长的数据类型（如某个组合的元组，或者某个函数签名）定义一个别名，使用别名可以让代码更简洁清晰，定义数据类型别名的语法：

* `类型 名称 = 数据类型`
* `type Name = DataType`

示例：

```js
类型 字符串 = 列表<字符>
类型 空型 = 结果<单元, 错误>
类型 排队令牌 = (整数, 字符串)
类型 整数过滤函数 = 逻辑 <- (整数)
类型 过滤函数<T> = 逻辑 <- (T)
```

```js
type String = List<Char>
type Void = Result<Unit, Error>
type Token = (Int, String)
type IntFilterFunc = Boolean <- (Int)
type FilterFunc<T> = Boolean <- (T)
```

使用类型别名可以缩短函数的定义语句，比如下面几个函数的定义的结果是一样的。

```js
## 直列式

函数 列表<整数> 筛选合格者(列表<整数> items, 逻辑 <- (整数) f)
    ...
以上

## 使用 `其中` 关键字

函数 列表<整数> 筛选合格者(列表<整数> items, 整数过滤函数 f) 其中
    整数过滤函数 = 逻辑 <- (整数)
    ...
以上

## 使用 `类型` 定义别名

类型 整数过滤函数 = 逻辑 <- (整数)
函数 列表<整数> 筛选合格者(列表<整数> items, 整数过滤函数 f)
    ...
以上
```

```js
## flat list

function List<Int> passFilter(List<Int> items, Boolean <- (Int) f)
    ...
end

## using 'where` keyword

function List<Int> passFilter(List<Int> items, IntFilterFunc f) where
    IntFilterFunc = Boolean <- (Int)
    ...
end

## using 'type' define alias name

type IntFilterFunc = Boolean <- (Int)
function List<Int> passFilter(List<Int> items, IntFilterFunc f)
    ...
end
```

### 字面两类型的自动（隐式）转换

当将一个能表示范围较小的数值的字面量赋值给一个范围较大的变量时，运行环境（的解析器）会进行类型的隐式转换（有些语言也叫类型提升）。

比如将一个整数（默认是 `Int`）的字面量赋值给 `Float` 类型的变量时，该字面量会被提升为 `Float` 类型。

示例：

`let Real r = 3`

会被自动替换为：

`let Real r = 3.0`

注意，如果将一个整数字面量（默认为 `Int64` 类型）赋值给诸如 `Int32`、或者 `Int8` 等类型的变量时，解析器也会事先进行检查，然后再赋值正确的数值给变量。

类型隠式转换仅发生在字面量赋值语句里，下面情况不会进行类型转换：

* 字面量之间的算术运算。
* 字面量作为参数传给函数；
* 不同类型变量的赋值；
* 字面量跟不同类型的变量之间的算术运算；
* 不同类型变量之间的算术运算。

XiaoXuan 的类型转换是发生在语法分析阶段，而不是在运行时。

当两个不同类型的**字面量**进行算术运算时（比如整数和浮点数之间的加法、减法等），其实不存在类型的隠式转换。其运算过程是：

1. 首先运行环境会获取第一个字面量的数据类型；
2. 然后查找该类型对应运算的函数；
3. 然后根据第二个操作数的类型寻找相应的方法重载；
4. 如果找到则调用该方法，如果找不到则抛出运行时错误。

示例：

`3 + 4.5`

首先运行环境会找到 `Int64::add` 方法，然后继续查找是否存在该方法的重载 `Int64::add(Int64, Real64)` 并调用它，即该加法表达式实际上被翻译为：

`Int64::add(3, 4.5)`

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
a = 456  # 错误
```

```js
let a = 123
a = 456  # Error
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

  # 以当前的累加值作为参数调用自己以
  # 再执行一遍当前这个过程
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

function a ()
    let x = 22           # 定义函数级的变量 x
    # let x = 99         # 当前环境已存在名称为 x 的变量，这里无法再定义变量 x

    for d in [1..10]     # 变量 d 的作用域仅限当前 for 语句块之内
        writeLine(x)     # 能输出函数级的变量 x = 22
        # let x = 33     # 当前环境仍然存在名称为 x 的变量，这里无法再定义变量 x

        let y = 33       # 定义语句块级的变量 y

        if d>0 then
            writeLine(x) # 能输出函数级的变量 x = 22
            writeLine(y) # 能输出语句块级的变量 y = 33

            let z = 44   # 定义内层语句块级的变量 z
        end

        # writeLine(z)   # 这里已经超出内层语句块级的变量 z 作用域

        let z = 55       # 当前环境不存在名称为 z 的变量，这里可以定义语句块级的变量 z
    end

    # writeLine(d)       # 这里已经超出 for 语句块变量 d 的作用域
end

function b ()
    # writeLine(x)       # 这里无法访问函数 a 里的变量 x
end
```

注意在有效作用域里不允许存在同名变量，比如下面的代码会引起运行时异常：

```js
函数 空型 测试()
    让 n = 123
    让 n = 456        # 当前环境已存在名称为 n 的变量，这里无法再定义变量 n

    如果 n > 100 那么 # 当前环境已存在名称为 n 的变量，这里无法再定义变量 n
      ...
    以上

    设有 i 取自 [1..10]
        ...
        让 n = 789    # 当前环境已存在名称为 n 的变量，这里无法再定义变量 n
        ...
    以上

    让 i = 666        # 这里已经超出循环语句块变量 i 的作用域，这里可以定义变量 i
以上
```

### 全局变量（::不支持）

全局变量其实就是作用域为模块级的普通变量，即在模块范围里（而不是在某个函数、类型实现、模块实现里）定义的变量。全局变量使用关键字 `定义` 来定义，其语法除了关键字不同，其余跟 `让` 语句一样。示例：

```js
模块 foo.bar

定义 整数 n = 123

@测试
函数 空型 第一个测试 ()
    书写行 (n) # 这里输出全局变量 n 的值
以上
```

```js
module foo.bar

define Int n = 123

@test
function Void firstTest ()
    writeLine (n) # output the value of global variable n
end
```

在模块范围定义的变量能够被其他模块所读取，只要被导入即可，也就是说它能被程序的任何地方所读取，所以也可以称为 "全局变量"。

示例：

```js
模块 hello.world

导入 foo.bar

@测试
函数 空型 第二个测试 ()
    书写行 (bar.n) # 这里输出 'foo.bar' 模块的变量 n 的值
以上
```

```js
module hello.world

import foo.bar

@test
function Void secondTest ()
    writeLine (bar.n) # output the value of global variable n which exists in the module 'foo.bar'
end
```

全局变量的赋值语句（即右值，一个字面量或者一个表达式）会在模块被加载时运行。准确来说，加载应用程序时，会先加载所有模块所有定义性质的语句，比如函数、结构体、特性、接口、组件实现等等的定义，然后才执行全局变量的赋值语句。

## 常量

因为 XiaoXuan 的变量的值不可变性，所以常量跟变量非常相近，不同的地方在于：

* 常量即使在某些支持变量可变的衍生版本里，也是不能被重新赋值；
* 常量的名称可以用在模式匹配里当作字面量使用，而变量名称在模式匹配里会被当成新的变量。

定义常量的语法是：

```js
常量 数据类型 常量名称 = 值
```

也可以一次定义一组：<!-- 考虑取消此特性，因为跟枚举的作用重复 >

```js
常量 数据类型 常量组名称
   成员名称1 = 值1
   成员名称2 = 值2
   ...
   成员名称N = 值N
以上
```

<!-- XiaoXuan 规定常量必须一组一组地定义，不能单独地使用一个名称定义一个常量值，-->
访问常量组的成员是，使用 `组名::成员名` 的格式。

示例：

```js
名称空间 http::client
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
        书写行 (ResponseCode::Ok) # 输出常量的实际值 "200"
        书写行 (ResponseCode::NotFound) # 输出 "404"
    以上
以上
```

```js
namespace http.client
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
        writeLine (ResponseCode::Ok)       # output the actual value "200"
        writeLine (ResponseCode::NotFound) # output "404"
    end
end
```

在模块里定义的常量可以在程序的任何地方所读取，所以也称为 "全局常量"，示例：

```js
名称空间 main
    函数 结果<单元, 错误> 第二个测试 ()
        导入 http::client::ResponseCode # 导入语句可以写在任何地方

        书写行 (ResponseCode::Ok) # 输出 "200"
        书写行 (ResponseCode::NotFound) # 输出 "404"
    以上
名称空间
```

```js
namespace main
    function Result<Unit, Error> secondTest ()
        import http.client.ResponseCode # Import statements can be written anywhere

        writeLine (ResponseCode::Ok) # output "200"
        writeLine (ResponseCode::NotFound) # output "404"
    end
end
```

注意：

<!--
* 导入全局常量时必须整组导入，不能只导入单独一个值，比如 `import http.client.ResponseCode` 是对的，但 `import http.client.ResponseCode.Ok` 是错误的。-->

* 常量组的组名不是一种数据类型，不能作为变量或者参数的数据类型，它仅仅是名称空间的一部分而已。

<!--
* 常量的值只能是基本的数据类型，或者结构体、联合体使用默认构造函数创建的实例，不能是一个表达式，即常量的右值不能是一表达式，比如一个函数的返回值。-->

<!--### 局部常量 -->

常量也可以在函数范围内定义，<!--比如有时可能只想在某一个函数里要一个严格只读的值，而不是想定义全局常量，--> 比如想定义一个临时的常量（即在地方地方用不着的常量）给模式匹配使用，对于在函数范围内定义的常量，就不能定义常量组了 <!--，可以不需要定义为一个组，即支持使用一个名称定义一个常量，语法：-->

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
        case Good: writeLine("Good") # 这里的 Good 将会是一个新的变量，其值等于变量 s
        case _: writeLine("Other") # 这行永远不会被执行
    end
end
```
