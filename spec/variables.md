# 变量

## 变量的声明和赋值

变量的声明和赋值需要同时进行，语法如下：

`让 数据类型 变量名称 = 值`
`let DataType name = value`

示例：

`让 整数 a = 123`
`let Int a = 123`

在 XiaoXuan 语言里，基本类型的数据是值，结构体、联合体也是值，一个函数也可以作为一个值，所以它们都可以赋值给一个变量。

### 类型推导

XiaoXuan 有类型推导机制，当值的类型可以确定的情况下，可以省略变量的数据类型声明，示例：

`让 a = 123`
`let a = 123`

变量 `a` 的数据类型会自动推导为 `整数`（`Int`）。

类型推导还具有传导性，函数的嵌套调用、函数的连续调用都会尽可能地推导出正确的数据类型。

一般情况下，对于变量声明及赋值语句：

* 如果是字面值赋值，**建议省略** 变量的数据类型声明，因为数据类型显而易见，省略数据类型声明不妨碍阅读理解且会更简洁；
* 如果是接收某个函数的返回值，**不建议省略** 变量的数据类型声明，因为调用其他模块的函数，其返回值的数据类型不明显，加上数据类型声明能让代码更清晰。
* 如果是接收范型的函数的返回值，当范型作用在返回值类型时，可以省略变量的数据类型声明，当范型是作用在参数时，则不建议省略。

### 列表、映射表、元组、函数的数据类型

* 列表（List）的数据类型由第一个元素决定，比如：

  `让 a = [1, 2, 3]`
  `let a = [1, 2, 3]`

  列表的数据类型为 `列表<整数>`（`List<Int>`）。

  `让 b = ["你好", "世界"]`
  `let b = ["Hello", "World"]`

  列表的数据类型为 `列表<字符串>`（`List<String>`）。

  因为空列表无法进行类型推导，所以要赋值一个空列表给变量，则需要声明变量的数据类型：

  `让 列表<字符串> b = []`
  `let List<String> b = []`

* 映射表（Map）的数据类型由第一个映射对的数据类型决定，比如：

  `让 a = {姓: "张", 名: "三"}`
  `let a = {firstName: "San", lastName: "Zhang"}`

  因为第一对映射对是：`姓->"张"`，映射表的数据类型为 `映射表<字符串, 字符串>`（`Map<String, String>`）。注意当映射表的字面量当中的键是字符串类型时，是可以省略包围字符串的双引号的，所以上例的映射表的完整写法是：

  `让 a = {"姓": "张", "名": "三"}`
  `let a = {"firstName": "San", "lastName": "Zhang"}`

* 元组可视为一个匿名的结构体，元组的数据类型由其中的各个元素（的顺序即数据类型）决定，比如：

  `让 a = (123, "你好", 真)`
  `let a = (123, "Hello", True)`

  则这个元组的数据类型是 `(整数, 字符串, 逻辑)`（`(Int, String, Boolean)`）

  因为元组常用于函数的返回值，而对于过程函数往往没有返回值，所以专门有一个叫做 `单元`（`Unit`）的数据类型，其值有且只有一个 `空`（`void`）。

* 函数的签名就是该函数的数据类型，比如下面的函数：

  `函数 整数 增加(整数 number, 整数 amount) = number + amount`
  `function Int increase(Int number, Int amount) = number + amount`

  它的签名是：

  `整数 <= (整数, 整数)`
  `Int <= (Int, Int)`

  其中 `<=` 符号用于分隔函数的返回值数据类型以及参数列表，表示这个表达式是一个函数签名。

  在字面上，函数的签名如同把函数的名称、参数的名称、函数主体通通全部移除之后的骨架，也就是说，函数签名只包含：

  * 函数返回值的数据类型
  * 各参数的数据类型的列表

  有时为了便于阅读理解，也可以保留函数的签名的参数名称，比如:

  `整数 <= (整数 number, 整数 amount)`
  `Int <= (Int number, Int amount)`

  对于范型（即类型参数化）函数，参数的数据类型就是范型参数，比如：

  `函数 T 增加(T source, E amount) =  source + amount`
  `function T increase(T source, E amount) =  source + amount`

  它的签名是：

  `T <= (T, E)`

  如果一个函数的参数的数据类型是函数，此时函数的定义语句会很长以至于影响阅读，比如：

  ```
  函数 返回值的数据类型 函数名称 (整数 <= (整数 a, 整数 b) 参数1, ..., 字符串 <= (字符串 s, 字符 c) 参数N)
      ...
  以上
  ```

  ```
  function DataType function_name (Int <= (Int a, Int b) param1, ..., String <= (String s, Char c) paramN)
      ...
  end
  ```

  可以在函数的主体之前将函数的签名定义成一个单独的名称，语法如下：

  ```
  函数 返回值的数据类型 函数名称 (签名名称1 参数1, ..., 签名名称N 参数N) 其中
      签名名称1 = 整数 <= (整数 a, 整数 b),
      签名名称N = 字符串 <= (字符串 s, 字符 c)
      ...
  以上
  ```

  ```
  function DataType function_name (type1 param1, ..., typeN paramN) where
      type1 = Int <= (Int a, Int b),
      typeN = String <= (String s, Char c)
      ...
  end
  ```

  语法当中的 `其中`（`where`） 关键字接着一系列参数类型的声明，多个类型声明之间使用逗号分隔，最后一个类型声明之后则是函数的主体开始。

### 定义数据类型别名

可以为名字较长的数据类型（如某个组合的元组，或者某个函数签名）定义一个别名，使用别名可以让代码更简洁清晰，定义数据类型别名的语法：

`类型 名称 = 数据类型`
`type Name = DataType`

示例：

```
类型 字符串 = 列表<字符>
类型 空值 = 结果<单元>
类型 排队令牌 = (整数, 字符串)
类型 整数过滤函数 = 逻辑 <= (整数)
类型 过滤函数<T> = 逻辑 <= (T)
```

```
type String = List<Char>
type Void = Result<Unit>
type Token = (Int, String)
type IntFilterFunc = Boolean <= (Int)
type FilterFunc<T> = Boolean <= (T)
```

使用类型别名可以缩短函数的定义语句，比如下面几个函数的定义的结果是一样的。

```
// 直列式

函数 列表<整数> 筛选合格者(列表<整数> items, 逻辑 <= (整数) f)
    ...
以上

// 使用 `其中` 关键字

函数 列表<整数> 筛选合格者(列表<整数> items, 整数过滤函数 f) 其中
    整数过滤函数 = 逻辑 <= (整数)
    ...
以上

// 使用 `类型` 语句

类型 整数过滤函数 = 逻辑 <= (整数)
函数 列表<整数> 筛选合格者(列表<整数> items, 整数过滤函数 f)
    ...
以上
```

```
// flat list

function List<Int> passFilter(List<Int> items, Boolean <= (Int) f)
    ...
end

// using 'where` keyword

function List<Int> passFilter(List<Int> items, IntFilterFunc f) where
    IntFilterFunc = Boolean <= (Int)
    ...
end

// using 'type' statement

type IntFilterFunc = Boolean <= (Int)
function List<Int> passFilter(List<Int> items, IntFilterFunc f)
    ...
end
```

## 变量的不可变性

变量定义并赋值之后，便无法再次赋值，也就是说，变量的值是不可变的。比如下面的语句会抛出运行时异常：

```
让 a = 123
a = 456  // 错误
```

```
let a = 123
a = 456  // Error
```

因为变量的值无法更改，所以对于我们所熟悉的（在可变变量的语言里）常用数据处理方法，在 XiaoXuan 里需要稍微转换一下。

比如有一个程序：让用户多次输入数字，然后计算数字的总和，直到用户输入 "Q" 为止。下面的是 JavaScript 版本：

```JavaScript
let sum = 0;
while(true) {
    let s = prompt('Enter a number:');
    if (s === 'Q') break;
    let i = parseInt(s);
    sum = sum + i;
    console.log(sum)
}
```

下面是 XiaoXuan 版本：

```
function Void add(Int sum)
  let s = readLine()?
  if s == "Q" then return Ok(void)
  let i = parse<Int>(s)
  let c = sum + i
  writeLine(c)

  add(c) // 调用自己（递归）以再执行一遍当前这个过程
```

XiaoXuan 语言会对函数的尾调用进行优化，只要函数最后执行的表达式是调用函数自己本身，则不会创建新的调用栈，也就是说不会堆栈溢出。

下面是使用 `for` 语句的示例：

```
let sum =
    for let Int acc = 0
        let s = readLine()?
        if s == "Q" then
            acc
        else
            let i = parse<Int>(s)
            let c = acc + i
            loop c
        end
    end

writeLine(c)
```

`for` 语句实际上是创建了一个匿名函数，然后通过 loop 语句调用自己，以实现 "循环" 的效果。

### 值的不可变性

XiaoXuan 的值（包括各种集合、结构体、联合体等）也是不可变的。比如

* 对于一个列表，直接修改它的项目的值是不允许的，增加或者删除项目都会返回一个新的列表；
* 对于一个结构体，无法修改它的成员的值，如果确实需要修改某个成员的值，则会返回一个新的结构体。

跟变量不可变性类似，在值可变语言里我们所熟悉的常用数据处理方法，在 XiaoXuan 里需要稍微转换一下。在某些 XiaoXuan 的衍生语言（比如 XiaoXuan Script）里可能会支持可变变量，但值不变性仍然会被保留。

比如某个结构体数据类型的变量，在被重新赋值之后，该变量原先指针所指的那份在内存（堆）中的数据仍然保留者，只是变量指向了一份新的结构体数据。当程序再次读取这个变量时，获取的是最新的版本。至于旧的版本，如果没有其他程序引用，它将被垃圾回收。

可见处理可变变量的方式总是 "只增" 新版本，然后把变量的指针指向新版本，而不是直接修改（或覆盖）原先的数据。这种处理方式类似源码的版本控制工具的工作方式，即默认情况下更新仓库实际上是增加新版本，所有历史版本都会保留者，不会因为某次仓库更新而把历史版本抹除了。

值得不可变性使得值可以放心传递值给多线程，不会因为某个线程修改了值而导致数据竞争。

### 变量的作用域

每个变量都有其作用域，比如：

* 在模块定义的变量，其作用域是其所在的模块（同一个模块可以分布在不同的源代码文件里）；
* 在函数里定义的变量，其作用域在其所在的函数里；
* 在语句块（包括条件语句、循环语句、匿名函数主体等）定义的变量，其作用域仅限其所在的语句块里。

示例：

```
module apple

define val n = 11 // 定义模块级的变量 n

define func Result<Unit> a ()
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

define func Result<Unit> b ()
    writeLine(n) // 输出模块级的 n = 11
end
```

注意在同一层作用域里定义同名的变量是不允许的，比如下面的代码会抛出运行时异常：

```
函数 空值 测试()
    让 n = 123
    如果 n > 100 那么
      ...
    以上

    让 n = 456 // 这句会引起运行时异常
以上
```

```
function Void test()
    let n = 123
    if n > 100 then
      ...
    end

    let n = 456 // Runtime error
end
```

### 全局变量（可能不支持）

全局变量其实就是作用域为模块级的普通变量，即在模块范围里定义的变量，跟定义普通变量一样，使用关键字 `让` 来定义。示例：

```
模块 foo.bar

让 整数 n = 123

@测试
函数 空值 第一个测试 ()
    输出行 (n) // 这里输出全局变量 n 的值
以上
```

```
module foo.bar

let Int n = 123

@test
function Void firstTest ()
    writeLine (n) // output the value of global variable n
end
```

在模块范围定义的变量能够被其他模块所读取，只要被导入即可，也就是说它能被程序的任何地方所读取，所以也可以称为 "全局变量"。

示例：

```
模块 hello.world

导入 foo.bar

@测试
函数 空值 第二个测试 ()
    输出行 (bar.n) // 这里输出 'foo.bar' 模块的变量 n 的值
以上
```

```
module hello.world

import foo.bar

@test
function Void secondTest ()
    writeLine (bar.n) // output the value of global variable n which exists in the module 'foo.bar'
end
```

## 常量

因为 XiaoXuan 的变量的值不可变性，所以常量跟变量非常相近，不同的地方在于：

* 变量能够被（不同作用域的其他同名变量）覆盖；
* 变量在某些 XiaoXuan 衍生语言里能被重新赋值；
* 常量不能被覆盖；
* 常量不能被重新赋值；
* 常量的名称可以用在模式匹配里当作字面量使用，而变量名称在模式匹配里会被当成新的变量。

定义常量的语法是：

```
常量 数据类型 常量组名称
   成员名称1 = 值1
   成员名称2 = 值2
   ...
   成员名称N = 值N
以上
```

XiaoXuan 规定常量必须一组一组地定义，不能单独定义一个常量值，访问的时候使用 `组名.成员名` 的格式。

示例：

```
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

```
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

function Result<Unit> firstTest ()
    writeLine (ResponseCode.Ok) // output the actual value "200"
    writeLine (ResponseCode.NotFound) // output "404"
以上
```

在模块里定义的常量可以在程序的任何地方所读取，所以也称为 "全局常量"，示例：

```
模块 main

函数 结果<单元> 第二个测试 ()
    导入 http.client.ResponseCode // 导入语句可以写在任何地方

    输出行 (ResponseCode.Ok) // 输出 "200"
    输出行 (ResponseCode.NotFound) // 输出 "404"
以上
```

```
module main

function Result<Unit> secondTest ()
    import http.client.ResponseCode // Import statements can be written anywhere

    writeLine (ResponseCode.Ok) // output "200"
    writeLine (ResponseCode.NotFound) // output "404"
end
```

有时可能只需一个常量值，比如在模式匹配里，想定义一个临时的常量（即在地方地方用不着的常量）使用，对于定义只有一个成员的常量组有简化的定义语法：

`常量 数据类型 组名.常量名称 = 值`
`const DataType GroupName.ConstantName = value`

示例：

```
const SomeType.Good = 100

match s
    case Score.Good: writeLine("Good")
    case _: writeLine("Other")
end
```

注意在模式匹配中，不能使用变量来代替常量：

```
let Good = 100

match s
    case Good: writeLine("Good") // 这里的 Good 将会是一个新的变量，其值等于变量 s
    case _: writeLine("Other") // 这行永远不会被执行
end
```

注意：

* 导入常量时必须整组导入，不能只导入单独一个值，比如 `import http.client.ResponseCode` 是对的，但 `import http.client.ResponseCode.Ok` 是错误的。
* 常量组名不是一种数据类型的名称，不能作为变量或者参数的数据类型，它仅仅是名称的一部分而已。
