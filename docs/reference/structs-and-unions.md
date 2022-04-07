# 结构体和联合体

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [结构体和联合体](#结构体和联合体)
  - [结构体](#结构体)
    - [实例化结构体](#实例化结构体)
    - [访问结构体成员的值](#访问结构体成员的值)
    - [更新成员的值](#更新成员的值)
    - [数据验证约束](#数据验证约束)
    - [自定义构造函数](#自定义构造函数)
    - [匿名成员结构体](#匿名成员结构体)
    - [无成员结构体](#无成员结构体)
  - [联合体](#联合体)
    - [实例化联合体](#实例化联合体)
    - [访问联合体成员的值](#访问联合体成员的值)
  - [元组](#元组)
    - [元组的访问](#元组的访问)
    - [添加元组元素](#添加元组元素)
  - [枚举](#枚举)
  - [数据类型的内部实现](#数据类型的内部实现)
    - [基本数据类型](#基本数据类型)
    - [联合体](#联合体-1)
    - [枚举](#枚举-1)
    - [值的复制](#值的复制)

<!-- /code_chunk_output -->

结构体和联合体是用户自定义数据类型。

XiaoXuan 各种数据类型之间没有继承关系。

## 结构体

结构体的作用是将一堆相关的数据打包成为一个整体。结构体由基本数据类型，比如整数、字符串、逻辑等组成，当然结构体也能嵌套其他结构体和联合体。结构体的定义语法如下：

```js
结构体 名称
    数据类型1 成员名称1
    数据类型2 成员名称2
    ...
以上
```

```js
struct Name
    DataType1 memberName1
    DataType2 memberName2
    ...
end
```

结构体的定义也可以写成一行的形式，只需把所有成员（member）用括号包围起来，然后使用逗号分隔各个成员即可，语法如下。

```js
结构体 名称(数据类型1 成员名称1, 数据类型2 成员名称2, ...)
```

```js
struct Name(DataType1 memberName1, DataType2 memberName2, ...)
```

示例：

```js
结构体 用户
    整数 id
    字符串 name
    逻辑 checked
以上

结构体 登录(用户 user, 时间 time)
```

```js
struct User
    Int id
    String name
    Boolean checked
end

struct Login(User user, Time time)
```

> 结构体的成员是有区分顺序的，即使成员的名称和类型相同，只要顺序不同，则结构体也不同。比如 `struct User(Int id, String name)` 跟 `struct User(String name, Int id)` 是不同的结构体，所以当重构代码时，如果调整了成员的顺序，则需要保证所有使用到该结构体的模块都要重新编译。

> 结构体成员的数据类型只能是基本的数据类型、结构体、枚举、联合体等，不能是函数（函数签名）、特性、接口等。

### 实例化结构体

:: TODO 尚未决定

运行环境会自动为每一个结构体创建一个默认构造函数，函数的参数为全部成员。使用 `新建`（`new`） 关键字可以调用此构造函数以创建结构体的实例。

示例：

```js
# 使用 new 关键字
让 u1 = 新建 用户 (100, "张三", true)
让 u2 = 新建 用户 (id=101, name="李四", checked=false)

# 原始函数调用
让 u1 = 用户::新建 (100, "张三", true)
让 u2 = 用户::新建 (id=101, name="李四", checked=false)

# 使用符号
让 u1 = 用户!(100, "张三", true)
让 u1 = 用户!(101, "李四", false)

# 使用符号 # 方案 2
让 u1 = 用户{100, "张三", true}
让 u1 = 用户{id: 101, name: "李四", checked: false}
```

```js
let u1 = new User (100, "foo", true)
let u2 = new User (id = 101, name = "bar", checked = false)

let u1 = User::new (100, "foo", true)
let u2 = User::new (id = 101, name = "bar", checked = false)

let u1 = User!(100, "foo", true)
let u1 = User{id: 101, name: "bar", checked: false}
```

跟普通函数一样，构造函数也可以按参数名称调用。

示例：

```js
让 u1 = new 用户 (id=100, name="张三", checked=true)
```

需注意的是，混合按参数顺序和按参数名称调用时，必须先写完所有按顺序传参的值，才可以开始写按名称传参的值。

### 访问结构体成员的值

使用 "实例名称 + . + 成员名称" 的格式可以访问结构体各个成员的值。

示例：

```js
让 编号 = u1.id
让 姓名 = u1.name
```

```js
let id = u1.id
let name = u1.name
```

### 更新成员的值

因为 XiaoXuan 的值不可变性，所以没法直接为结构体的成员赋予新值，比如 `u1.name = "John"` 这语句是语法错误的，如果需要更新成员的值，只能新建一个结构体实例，然后把原结构体实例需要保留的值一一复制过来，示例：

```js
让 u3 = 用户(u1.id, "王五", u1.checked)
```

```js
let u3 = User(u1.id, "John", u1.checked)
```

上面的代码的作用相当于将 `u1` 的 `name` 成员的值更新，然后将新实例存储到变量 `u3`。

XiaoXuan 支持一种简化的更新结构体成员（即复制其他实例成员的值）的语法，调用构造函数时，使用按参数名称方式传入成员的新值，并使用 "..." 符号（即三个点）复制原实例成员的值。

示例：

```js
让 u3 = 用户(name="王五", ...u1)
让 u4 = 用户(name="赵六", checked=false, ...u1)
```

```js
let u3 = User(name="John", ...u1)
let u4 = User(name="Smith", checked=false, ...u1)
```

### 数据验证约束

有时一个结构体的成员的值有一定的使用限制，比如表示有理数的结构体的分母不能为 0 等。可以在定义结构体时添加 `@验证`（`@validate`）标注用于约束成员的值。示例：

```js
@验证(id > 0 :并且 name != "")
结构体 用户
    整数 id
    字符串 name
    逻辑 checked
以上
```

```js
@validate(id > 0 :and name != "")
struct User
    Int id
    String name
    Boolean checked
end
```

标注 `@验证` 的内容是一个能返回逻辑值的表达式，该表达式会被运行环境自动置入默认构造函数当中，在构造结构体实例时，如果该表达式返回 `假`，则会引起运行时异常，即实例构造失败。

一个结构体可以有多个 `@验证` 标注，只要任何一个验证表达式返回 `假`，都会导致实例构造失败。

标注 `@验证` 也可以标在结构体的成员之上，示例：

```js
结构体 用户
    @验证(id > 0)
    整数 id

    @验证(name != "")
    字符串 name

    逻辑 checked
以上
```

当标注 `@验证` 标在成员之上时，只能单独验证该成员的值。

### 自定义构造函数

运行环境会自动为结构体创建一个默认构造函数，函数的参数为全部成员。可以为结构体添加一个或多个 `新建`（`new`）方法，只要这些方法的参数列表跟默认构造函数不同，且返回值为该结构体，则它们都会自动称为构造函数。当程序使用 `新建`（`new`）关键字实例化结构体时，会根据参数情况自动选择适当的构造函数。

示例：

```js
实现 用户
    函数 新建 (整数 id, 字符串 name) = 新建 用户(id, name, true)
    函数 新建 (整数 id) = 用户(id, "张三")
以上
```

```js
implement User
    function new (Int id, String name) = new User (id, name, true)
    function new (Int id) = User(id, "张三")
end
```

上面定义了两个自定义构造函数。其中第一个允许用户省略 `checked` 参数，函数的主体为调用默认的构造函数。第二个构造函数则允许用户省略 `name` 和 `checked` 参数。

通过自定义构造函数，我们可以简化某些复杂的结构体的实例化过程。

需要注意的是，默认构造函数是无法覆盖的，即我们无法添加一个签名跟默认构造函数一样的函数。即运行环境会自动为每个结构体生成类似下面的函数：

```js
实现 用户
    函数 新建 (整数 id, 字符串 name, 逻辑 checked)
        ...
    以上
以上
```

该函数会调用运行环境提供的内置方法以创建结构体实例。从中也可以看出，创建实例除了使用 `创建`（`new`） 关键字，其实也可以直接调用结构体的 `创建`（`new`）函数。示例：

* 让 u1 = 用户.创建 (123, "foo", true)
* let u1 = User.new (123, "foo", true)

### 匿名成员结构体
### 无成员结构体

## 联合体

联合体由一个或多个成员组成，每一个成员可以是一个结构体、一个元组、一个常量。

虽然联合体可以有多个成员，但一个联合体实例的值只能是其中一个成员的值。

需注意的是，联合体成员的定义必须在现场定义，而不能使用外部已定义的（比如不能使用外部定义的结构体）。联合体的定义语法是：

```js
01  联合体 名称
02      成员名称1 (数据类型1 成员名称1, 数据类型2 成员名称2, ...)
03      成员名称2 (数据类型1, 数据类型2, ...) // ::TODO 考虑不支持无成员名称的成员
04      成员名称3
05      ...
06  以上
```

```js
01  Union Name
02      MemberName1 (DataType1 memberName1, DataType2 memberName2, ...)
03      MemberName2 (DataType1, DataType2, ...) // ::TODO removed?
04      MemberName3
05      ...
06  end
```

上面语句的 02 行定义了一个结构体类型的成员，03 行定义了一个元组类型的成员（::TODO 考虑不支持无成员名称的成员），04 行定义了一个常量，需注意的是这个常量并不需要指定其数值，这点跟其他面向对象语言当中的 "枚举" 类型有些类似。

示例：

```js
联合体 作品
    书籍(字符串 title, 字符串 isbn)
    专辑(字符串 title, 字符串 artist)
以上
```

```js
union Work
    Book(String title, String isbn)
    Album(String title, String artist)
end
```

> 联合体的成员是有区分顺序的，即使成员的名称和类型相同，只要顺序不同，则联合体也不同。比如 `union One(...) Two(...)` 跟 `union Two(...) One(...)` 是不同的联合体，所以当重构代码时，如果调整了成员的顺序，则需要保证所有使用到该联合体的模块都要重新编译。

### 实例化联合体

联合体的结构体类型和元组类型的成员的实例化跟正常的结构体和元组的实例化方法一样，至于常量，则不需实例化，直接当成常量使用即可。

示例：

```js
让 作品1 = 作品::书籍("从地球到月球", "123456")
让 作品2 = 作品::专辑("Fly Me to the Moon", "foobar")
```

```js
let work1 = Work::Book("从地球到月球", "123456")
let work2 = Work::Album("Fly Me to the Moon", "foobar")
```

### 访问联合体成员的值

因为联合体实例只能存储其中一个成员的值，所以首先需要确定它的值是哪个成员，然后再读取其成员的值。XiaoXuan 只支持通过模式匹配来完成这个任务。示例：

```js
匹配 作品1
    情况 书籍(title, isbn):
        ...
    情况 专辑(title, artist):
        ...
以上
```

```js
match work1
    case Book(title, isbn):
        ...
    case Album(title, artist):
        ...
end
```

如果有时只对其中一个成员的值感兴趣，也可以使用简化版的 `匹配` 语句 ———— `如果 让` 语句来完成，示例：

```js
如果 让 书籍(title, isbn) 匹配 作品1 那么
    ...
以上
```

```js
if let Book(title, isbn) match work1 then
    ...
end
```

## 元组

`元组`（`Tuple`）<!-- 考虑取消这个特性  是 XiaoXuan 函数传参的底层原理，也就是说，传一组参数给一个函数，实际上是传了一个由一个或多个数值组成的元组给函数。一个函数有且只有一个参数和一个返回值。 -->

元组一般用于函数需要返回多个数值（却又不想单独创建一个专门的结构体）的情况，即使用元组把多个值包装成一个值然后返回。

一个元组可以视为一个 **匿名成员的结构体**，即它由固定数量、顺序、数据类型的一个或多个成员组成。注意跟其他语言的元组不同，XiaoXuan 的元组并不是一种集合。

元组的字面量的格式是：使用一对括号包围所有数据，数据之间使用逗号分隔。

示例：

```js
(1, 2, 3)
(1, "hello", true, 3.14)
```

如果元组成员是表达式的返回值，则可以分行写，示例：

```js
("hello world",
1+2*3,
someFunction(1,2),
)
```

元组最后一个成员后面可以添加逗号，也可以不添加。有些人习惯添加上逗号是为了让每一行看起来格式一致，同时当以后需要增加或者删除成员时，使用版本管理软件的比较（Diff）源码时，更加能准确被修改的行。

元组的数据类型为元组各个成员的数据类型的组合，比如上面两个元组的数据类型分别是：

* `(整数, 整数, 整数)`
* `(整数, 字符串, 逻辑, 浮点数)`

需注意，列表要求所有元素的数据类型必须相同，而元组的每个成员的数据类型都可以不同。

> 元组无法使用函数构建。

为了一致性，也存在单独一个成员的元组，其字面量是在第一个数值后面加上一个逗号。

示例：

* `(456, )`
* `("foo", )`

上面两个元组的数据类型分别是 `(整数)` 和 `(字符串)`。但作为函数的参数整体时，并不需要在后面加一个逗号，比如 `let a = sqrt(36)`，解析器会自动把 `(36)` 解析为元组。但在其他场合，这对括号仅仅被解析为 "语句块"，然后被求值，最后得一个整数 `36`。

对于一个里面仅有一个数值（且没有逗号）的一对括号：

* 如果写在函数的后面，它会被解析为元组。
* 写在其他地方会被解析为语句块。

<!--
::TODO 考虑取消

另外还有空元组 `()`，它是 `Unit::Empty` 值的字面量。它用在几种场合：

* 调用一个无参数的函数，比如 `doSomething()`
* 当一个函数的参数数据类型为 `Unit` 时，可以传入 `Unit::Empty`，也可以传入 `()`，比如 `Result::Ok(())`；
* 当一个函数的返回值类型为 `Unit` 时，返回语句可以写 `return Unit::Empty`，也可以写 `return ()`，甚至直接写成 `return`。
-->

### 元组的访问

元组跟列表和映射表不同，它的成员既不能通过位置/索引来访问，也没法通过成员名称来访问，一般使用模式匹配或者模式解构来访问。

示例，假设现有元组 `a`，其值为：

`让 a = (123, "hello", true)`

可以使用如下语句获取它的三个成员的值：

`让 (a,b,c) = a`

三个变量 `a`, `b`, `c` 的值分别为 123, "hello" 和 true。

除了模式匹配和模式解构，还可以通过 <!-- 运行环境自动为元组生成的成员名称来访问，成员名称的分别为 "_1"，"_2" 如此类推，即一个下划线加上成员的次序，-->
`获取成员`（`getMember`）函数获取成员的值，向函数传入成员的次序即可读取相应的成员值，次序从数字 1 开始（而不是从 0 开始）。

示例：

```js
让 a = (123, "hello", true)

让 m1 = a.获取成员(1) # 123
让 m2 = a.获取成员(2) # "hello"
让 m3 = a.获取成员(3) # true
```

使用 `成员个数`（`memberCount`） 函数可以获取成员的个数。

示例：

```js
让 c = a.成员个数() # 3
```

<!-- 考虑去除此特性
需注意的是，在构造元组实例时，运行环境支持为成员值附带上名称。比如在调用函数时，可以按参数位置传参，也可以按参数名称传参，或者混合两种传参方式。但成员的名称仅仅用于构造元组实例，只是成员的一个附加的数据，XiaoXuan 没提供获取成员名称的方法，也没提供按元组成员名称访问成员的方法。

示例：

```js
让 a = (1, 2, 3, 名称 = "hello", 形状 = "circle")

让 m4 = a.获取成员(4) # "hello"
让 m5 = a.获取成员(5) # "circle"
```

元组成员大致相当于如下的联合体：

```js
union TupleMember<T>
    Value(Int index, T v)
    ValueWithName(Int index, String name, T v)
end
```
-->

### 添加元组元素

<!-- 考虑取消此特性 -->

元组常用的只有构造和读取两种操作，一般很少需要修改元组的数据，不过<!-- XiaoXuan 还是提供了添加新成员到一个元组的头部的方法，用于构造一个新元组。--> 可以通过是使用一对圆括号和 "..." 符号（三个点号）的方法向元组头部添加成员。
<!--
示例：

```js
让 a = (1, 2, 3)
让 b1 = (99, ...a)
让 b2 = (77, "foo", true, ...a)

# b1 == (99, 1, 2, 3)
# b2 == (77, "foo", true, 1, 2, 3)
```

这种语法对应的函数是 `元组::添加`（`Tuple::add`），它是一个系统函数，有且只有一个参数，可以接受任意类型的值，另外还有 `元组::追加`（`Tuple::append`） 系统函数用于向元组末尾添加成员。 -->

## 枚举

枚举跟全局常量相类似，不过枚举有以下几个特点：

* 枚举的成员数量固定；
* 枚举的成员的值无需指定，运行环境内部按照枚举成员的定义顺序自动分配从 `整数` 数值 `0` 开始分配数值；
* 枚举是一种数据类型；
* 枚举的值无法直接跟其他数据类型转换，也就是说无法从枚举值转为 `整数`，也无法直接从 `整数` 转成枚举值。

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
    书写行 (c)
以上

# 调用函数
设置背景色 (原色.红)
```

```js
function setBackgroundColor (PrimaryColor c)
    writeLine (c)
end

# call function
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

> 枚举的成员是有区分顺序的，即使成员的名称相同，只要顺序不同，则枚举也不同。比如 `enum Red Blue` 跟 `enum Blue Red` 是不同的枚举，所以当重构代码时，如果调整了成员的顺序，则需要保证所有使用到该枚举的模块都要重新编译。

<!-- 考虑移除

可以使用 @enumValue 标注指定枚举成员的具体值，不过一般没必要这样做，因为语言不提供读取成员内部值（一个整数）的方法。另外编译器不检查指定的成员值是否有重复，所以需要用户确保成员值是正确的。

这个标注的作用是当一个枚举数据类型需要（通过 API 或者序列化）跟外部程序或者本地库（比如 C/C++/Rust 等语言所生成的库）进行内存级别的运算时才有实际意义。比如基本数据类型 `逻辑`（`Boolean`）的定义如下：

```js
enum Boolean
    @enumValue(0)
    False

    @enumValue(1)
    True
end
```
-->

## 类型的内部实现

XiaoXuan 的类型系统包括数据类型（简称类型）、函数类型（也叫做函数签名）两种。

注意数据类型的检查位于编译阶段，在 IR 层面及在运行过程中均不作数据类型检查。也就是说 XiaoXuan 是静态数据类型编译型语言。

### 数据类型

在运行环境内部，所有数据类型（包括结构体、联合体、元组、枚举）本质上都是结构体，程序使用到的所有结构体都会登记在一个 `数据类型记录表` 里。这个记录表是一个有序表，一般有以下几个部分组成：

1. 成员表。成员表是一个有序数组，可以使用下标访问成员。
   数据的每一个元素由两个数据组成：
   * 数据类型编号。
   * 成员名称

成员表的第一条记录（即编号 0）是一个空数组。

2. 信息表
   * members 成员表编号。如果当前数据类型没有成员（即空结构体）或者是虚拟机基本数据类型，则该字段值为 0。
   * category 数据类型的类别，可能的值有：
     - 结构体，即一般结构体
     - 联合体结构体
     - 联合体成员结构体
     - 枚举结构体
     - 元组
     - 基本数据类型
   * name 数据类型名称，比如 "Int64"，注意一个数据类型可以定义别名，所以存在多个类型指向同一个成员表的情况；
   * namePath 所在的名称空间，比如 "core"；
   * fullName 包括命名空间路径和名称的全称，比如 "core::Int64"；
   * parent 联合体结构体指针。仅当当前类型为联合体成员结构体时这个字段值才有意义，除此之外该字段值为 0。

信息表的第一条记录（即编号 0）是一个空元组。

可以使用宏 typeOf(...) 获取一个数据类型的详细信息，宏返回一个 `Type` 结构体，该结构体的定义如下：

```js
struct Type
    // Option<Array<Member>> members
    TypeCategory typeCategory
    String name
    String namePath
    // String fullName
    Option<Type> parent
end

enum TypeCategory
    Struct
    Union
    UnionMember
    Enum
    Tuple
    Primitive
end
```

可以通过 `Type::getMembers` 方法获取类型的成员列表。

```js
struct Member
    Type type
    String name
end
```

#### 基本数据类型的实现

XiaoXuan 的基本数据类型（比如整数、实数）在标准库里定义为结构体，所以在运行环境内部的 `数据类型记录表` 里也有其相关记录。

下面是标准库里对于 `Int32` 类型的定义：

```js
@compileTypeCategory(TypeCategory::Primitive)
@compileTypeNative(type="i32") // 编译指令，用户不可使用
struct Int32
    // empty
end
```

上面的 `@compileTypeNative` 标记用于告诉编译器，当一个基本数据类型的变量在编译成 IR 时会被解析为虚拟机基本数据类型，其类型信息将会被丢弃。也就是说对于基本数据类型的变量值，在运算过程中其值是存储在寄存器即栈当中，当遇到 `typeOf` 一个基本数据类型时，编译器实际上会把它翻译成一个内部的函数 `typeByNumber` 直接从类型记录表里读取类型信息。

#### 一般结构体的实例的实现

一个结构体变量储存的是一个指向该结构体数据（一个内部结构体）的指针，该内部结构体的信息如下：

* type
  数据类型编号，也就是目标数据类型在 `数据类型记录表` 的索引值；
* value_ref
  - 当数据类型是虚拟机基本数组类型时，该值是一个指向实际值的指针。
  - 当数据类型是一个结构体时，该值是一个指向结构体各成员情况的指针。

结构体成员是一个数组，数组的每个成员跟上面结构体一样，该数组如下：

[{type, value_ref}, {type, value_ref}, ...]

> 跟一个基本数据类型的变量的情况不同，如果一个结构体的成员是基本数据类型，其值也是保存在堆（heap）里。

#### 联合体的实现

联合体使用两次结构体实现，即联合体本身会被解析为一个结构体

假设有如下一个联合体：

```js
union Option
    Some(Int value)
    None
end
```

编译器会自动生成如下结构体及其常量值：

```js
namespace std
    @compileTypeCategory(TypeCategory::Union)
    @derive(std::trait::Union)
    struct Option
        WordWidth memberNumber
        WordWidth memberAddr
    end

    namespace Option
        // 构造结构体型成员
        function Parent::Option new(WordWidth memberNumber, WordWidth memberAddr)
            // native
        end

        // 构造常量型成员
        function Parent::Option new(WordWidth memberNumber)
            // native
        end

        @compileTypeCategory(TypeCategory::UnionMember)
        @compileTypeParent("std::Option")
        @compileMemberNumber(0)
        struct Some
            Int value
        end

        // 构造成员 None
        @compileTypeCategory(TypeCategory::UnionMember)
        @compileTypeParent("std::Option")
        @compileMemberNumber(1)
        const None = new(1)

        // 构造成员 Some
        function Parent::Option Some(Int value)
            let some = Current::Some::new(value)
            let addr = Pointer(some)
            new(0, addr)
        end

        function equal(Parent::Option left, Parent::Option right)
            // 自动生成
            // 先比较 memberNumber
            // memberNumber 相同时再逐个 member 比较，伪代码如下：
            // if left.memberNumber == right.memberNumber then
            //      switch left.memberNumber
            //          case 0:
            //              Current::Some::equal(*left.memberAddress, *right.memberAddress)
            //          case 1:
            //              1
            //      end
            // else
            //      false
            // end

        end

        namespace Some
            // 构造成员 Some 的实际函数
            function Parent::Some new(Int value)
                // native
            end

            function equal(Parent::Some left, Parent::Some right)
                // 自动生成
            end
        end
    end
end
```

#### 枚举的实现

枚举也是使用结构体实现。

假设有如下一个枚举：

```js
enum Color
    Red
    Green
    Blue
end
```

编译器会自动生成如下结构体及其常量值：

```js
@compileTypeCategory(TypeCategory::Enum)
@derive(std::trait::Enum)
struct Color
    Int Value
end

namespace Color
    const Color Red = Color::new(0)
    const Color Green = Color::new(1)
    const Color Blue = Color::new(2)
end
```

#### 元组的实现

::TODO

### 值的复制

XiaoXuan 默认使用数据的引用计数来实现资源/垃圾回收。变量是一个指向存放在堆中的数据的指针，当将一个变量赋值给另外一个变量，以及作为参数传递给一个函数时，该变量所指向的数据的引用数便增加一个；当变量离开其作用范围时引用数则减少一个。当数据的引用数减少到 0 时，运行时就会回收这个数据资源。

但有些数据类型是不使用引用计数来管理的，这些数据直接在栈里参与计算，如果被赋值给另外一个变量，则直接复制一份。这些数据类型有：

* 基本数据类型：整数、自然数、实数、逻辑、字符；
* 枚举

它们具有 `复制` 特性。

### 函数签名

::TODO

函数的签名由返回值（一个数据类型）和一个参数列表（一个元组数据类型）组成，函数的名称、各参数的名称都不属于函数签名的一部分，函数签名本身也是一种数据类型，也会登记在运行环境的 `数据类型记录表` 里，函数签名的第一个成员是返回值的数据类型，第二个成员是参数列表（元组数据类型）。

> 函数的泛型、参数默认值（可选参数）、模式表达式等信息，在翻译成 IR 时已经会被丢弃（源码与 IR 码的映射信息存储在其他地方，不在 IR 代码里）。
