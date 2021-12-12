# 模式匹配和模式解构

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [模式匹配和模式解构](#模式匹配和模式解构)
  - [模式匹配](#模式匹配)
    - [`匹配`（`match`）语句](#匹配match语句)
      - [模式表达式](#模式表达式)
        - [作用域内存在同名变量](#作用域内存在同名变量)
        - [模式表达式存在同名变量](#模式表达式存在同名变量)
      - [类型匹配](#类型匹配)
      - [结构体匹配](#结构体匹配)
      - [部分匹配](#部分匹配)
      - [带条件的模式匹配](#带条件的模式匹配)
      - [带解析的模式匹配](#带解析的模式匹配)
      - [正则表达式模式匹配](#正则表达式模式匹配)
      - [匹配复合结构数据时保留原始值](#匹配复合结构数据时保留原始值)
    - [`如果 让...匹配` 语句](#如果-让匹配-语句)
    - [模式匹配函数 (NEW)](#模式匹配函数-new)
  - [模式解构](#模式解构)
    - [模式解构的异常](#模式解构的异常)
    - [列表的解构](#列表的解构)
    - [使用模式解构访问列表的元素(::DUPLICATED??)](#使用模式解构访问列表的元素duplicated)
    - [映射表的解构](#映射表的解构)
    - [结构体的解构](#结构体的解构)
    - [元组的解构](#元组的解构)

<!-- /code_chunk_output -->

## 模式匹配

模式匹配用于判断目标数据是否跟指定的数据模式相匹配，模式匹配发生于：

1. `匹配` 语句的 `情况` 关键字后面；
2. `条件` 语句的 `情况` 关键字后面的 `让` 表达式；
3. `分支` 语句的 `情况` 关键字后面的 `让` 表达式；
4. `如果 让 ... 匹配` 语句；
5. 模式函数。

模式匹配不单单用于判断是否匹配问题，同时也用于按照模式获取目标数据的值（通常把这种模式匹配称为 "解构"），或者 "同时判断部分数据的是否匹配，同时获取部分数据的值"。XiaoXuan 所有具有 "赋值" 性质的语句实际上都是模式匹配（解构），包括：

1. `让`（`let`）赋值语句
2. `现有 取自`（`for in`）循环语句 <!-- `遍历`（`iterate`）语句的 `到`（`to`）关键字之后、-->
3. `现有 让`（`for let`）循环语句的 "值初始化" 以及 "值更新"
4. 函数的参数传值、

### `匹配`（`match`）语句

`匹配` 语句会对 "待检查的数据" 的数据类型、结构、值等进行比较，只有完全匹配的分支才被执行。

示例：

```js
让 v = ("foo","bar","foo")

匹配 v
    情况 (a,b,c):
        "成功，a 的值将会是 'foo', b 是 'bar', c 是 'foo'"
    情况 ("foo", a, b):
        "成功，第一个元素是 'foo', a 的值将会是 'bar', b 是 'foo'"
    情况 (a,b,a):
        "成功，a 的值将会是 'foo', b 是 'bar', 第一个和第三个元素的值相同"
    情况 (a,...b):
        "成功，a 的值将会是 'foo', b 是 ('bar','foo')"
    情况 (a,_,_):
        "成功，a 的值将会是 'foo', 丢弃第二个和第三个元素的值"
    情况 (_,_,_):
        "成功，丢弃所有元素的值"
    情况 (a,a,b):
        "失败，第一个和第二个元素的值不相同"
    情况 (_,_):
        "失败，因为 v 有 3 个元素"
    默认:
        "总是成功"
end
```

```js
let v = ("foo","bar","foo")

match v
    case (a,b,c):
        "OK, the value of 'a' will be 'foo', 'b' is 'bar', 'c' is 'foo'"
    case ("foo", a, b):
        "OK, the first element is 'foo', the value of 'a' will be 'bar', 'b' is 'foo'"
    case (a,b,a):
        "OK, the value of 'a' will be 'foo', 'b' is 'bar', the first and third elements have the same value"
    case (a,...b):
        "OK, the value of 'a' will be 'foo', 'b' is ('bar','foo')"
    case (a,_,_):
        "OK, the value of 'a' will be 'foo', discard the values of the second and third elements"
    case (_,_,_):
        "OK, descard the values of all elements"
    case (a,a,b):
        "Failed, The first and second elements do not have the same value"
    case (_,_):
        "Failed, because 'v' has 3 elements"
    default:
        "Always OK"
end
```

在上面示例代码中：

* `匹配` 关键字后面的是待检查的数据
* 每一个 `情况` 关键字后面的是一个模式表达式，如果其中一个模式表达式匹配成功，则执行其分支的语句，然后跳出匹配语句。
* `默认` 关键字后面的是当上面的所有模式表达式都不匹配时，则执行默认部分的语句。
* 如果没有 `默认` 分支，且所有模式表达式都不匹配，则运行环境会抛出运行时异常。
* 匹配语句会返回匹配中的分支的语句（或表达式）的值。

> `模式` 语句是 `条件` 语句的语法糖。

#### 模式表达式

模式表达式由字面量（包括列表的中括号、元组的括号、映射表的花括号）、常量及变量等组成。

其中字面量和常量用于跟 "待检查的数据" 作相等比较，而变量则作为占位符，当匹配成功时（即字面量和常量相等、结构相同、元素或成员个数相同），变量则会捕获其所对应的数值。

##### 作用域内存在同名变量

如果模式表达式当中存在其 `匹配` 语句所在的作用域同名的变量名，因为 XiaoXuan 语言不允许在作用域内存在同名变量，所以这条模式表达式会引起语法错误。<!-- 模式表达式当中的变量会覆盖外面的变量，新变量的作用域包括该分支的代码。-->

示例：

```js
让 i = 123
让 v = (77,88)
匹配 v
    情况 (i, j):
        // 上面一行会引起语法错误，因为在作用域内已经存在名称为 `i` 的变量
以上
```

```js
let i = 123
let v = (77,88)
match v
    case (i, j):
        // syntax error,
end
```

##### 模式表达式存在同名变量

如有匹配式中存在两个或以上同名变量，则第一个会被赋值，第二个及之后的会当成常量来比较。示例：

```js
让 v = (11, 22, 22, 11)
匹配 v
    情况 (a, a, b, b):
        书写行 ("匹配失败")
    情况 (a, b, b, a):
        书写行 ("匹配成功，变量 a 的值将会是 11, b 是 22")
以上
```

```js
let v = (11, 22, 22, 11)
match v
    case (a, a, b, b):
        writeLine ("Failed")
    case (a, b, b, a):
        writeLine ("Ok, the value of 'a' will be 11, 'b' is 22")
end
```

上面例子中，当运行时检查第一个模式匹配表达式时，首先检查数据类型通过（都是元组），然后检查成员数量通过（都是 4），然后发现第一个成员是一个变量 `a`，则赋值 `11` 给它，然后发现表达式第二个成员还是变量，且名字是已经出现过的 `a`，这时就会取出 `a` 的值（也就是 `11`）跟实际数据（即 `22`）作相等比较，发现不通过，所以该表达式匹配失败。

<!--
## 赋值语句（模式解构）
示例，现有如下数据：

```js
让 v1 = (123, 456)
让 v2 = (77, 88)
让 vv = [v1, v2]
```

赋值语句解构：

```js
让 (a, b) = v1
```

遍历语句解构：

```js
遍历 vv 到 (a, b)
    ...
以上
```

`现有` 循环语句解构：

```js
现有 让 (a,b) = v1 如果 a>0 :并且 b>0 那么
    ...
    让 i = ...
    让 j = ...
    重复 (i,j)
以上
```
-->

### 赋值语句的模式解构

XiaoXuan 的赋值语句实质是模式解构，比如 `让 4 = 4` 语句是合法的（虽然没有任何意义）。当赋值语句的左边是一个单纯的变量时，因为变量能匹配任何值，所以其作用就是单纯的 "赋值"。当赋值语句左边不是一个单纯的变量时，则发生模式匹配。

需要注意的是，因为缺少 `匹配` 语句的 `默认` 分支，如果赋值语句的模式不匹配，会直接引起运行时异常。

示例：

```js
union User
    Student(String name)
    Teacher(String name)
end

let s = new User.Student("foo")
let Teacher t1 = s
let Teacher(name) = s
```

上面代码最后两行都会引起运行时异常，第一行很容易理解，跟很多编程语言一样，变量的类型不一致是不能赋值的，但 XiaoXuan 并不是简单地根据数据类型来判断，而是通过模式匹配来判断的。第二行则是标准的模式匹配失败。

再举一个数据类型匹配，但模式匹配失败的示例：

```js
让 (a, b, b, a) = (1, 2, 3, 4)
```

上面的赋值语句（模式解构）虽然左右两边的数据类型一致（都是元组），但因为模式不匹配，所以会引起运行时异常。

<!-- 模式解构可以应用于列表、映射表、结构体、元组等数据，但**无法解构**联合体，因为联合体的值是其多个成员的其中一个，只能通过模式匹配先匹配类型再解构，即只能使用 `匹配` 语句或者使用 `如果 让` 语句来获取其中的值。-->

### `让` 语句的返回值

模式结构语句在成功匹配时，语句的返回值是成功解构后的值，而匹配失败时，则抛出运行时异异常。

示例：

```
let v1 = (let a = 123)
let v2 = (let User(name) = new User("foo"))
let v3 = (let User(99, name) = new User(99, "foo"))
let v4 = (let u expand User(id, name) = new User(88, "bar"))
```

以上的 4 个变量的值分别为：

* 一个整数，值为 `123`
* 一个字符串， 值为 `"foo"`
* 一个元组，值为 `(99, "foo")`
* 一个 `(User, Int, String)` 元组，值为 `(User(88, "bar"), 88, "bar")`

### 列表的解构

示例：

```js
# 第一个 == 1, 第二个 == 2, 剩余 == [3,4,5,6]
让 [第一个, 第二个, ...剩余] = [1,2,3,4,5,6]

# 丢弃第一个和第二个元素的值， 第三个 == 3
让 [_, _, 第三个] = [1,2,3,4,5]

# 解构一个二维列表
# 第一个 == 1, 第二个 == 2, 第三个 == 3
让 [[第一个, 第二个], [第三个, _]] = [[1,2],[3,4],[5,6]]

# 使用索引来解构（::不支持）
# 注意索引从 1 开始，而不是从 0 开始
# x == 1, y == 6
让 [1:x, 6:y] = [1,2,3,4,5,6]
```

```js
# first == 1, second == 2, rest == [3,4,5,6]
let [first, second, ...rest] = [1,2,3,4,5,6]

# drop the first and the second element value, third == 3
let [_, _, third] = [1,2,3,4,5]

# deconstructing a two-dimensional list
# first == 1, second ==2, third == 3
let [[first, second], [third, _]] = [[1,2],[3,4],[5,6]]

# Deconstructing using indexes (::not supported)
# Note that indexes start at 1, not 0
# x == 1, y == 6
let [1:x, 6:y] = [1,2,3,4,5,6]
```

在上例中：

* 其中的 `...` 符号（三个点号）表示获取列表当中剩余的其他元素；
* 其中的 `_` 符号（下划线）表示仅匹配位置，丢弃其值。

### 使用模式解构访问列表的元素(::DUPLICATED??)

使用模式解构来获取列表的元素比使用函数的更加简单直观，示例：

```js
让 a = [1,2,3,4,5]

# 获取第 1 个元素并赋值给变量 i，此时 i == 1
让 [i] = a

# 获取第 1 和第 2 个元素分别赋值给变量 i 和变量 j，此时 i == 1, j == 2
让 [i, j] = a

# 获取第 1 和第 2 个元素，第 1 个元素的值丢弃，第 2 个元素的值赋值给 i，此时 i = 2
让 [_, i] = a

# 获取第 1 个以及剩余的元素，第 1 个元素赋值给变量 i，
# 剩余的元素（是一个列表）赋值给变量 j，此时变量 j == [2, 3, 4, 5]。
让 [i, ...j] = a

# 获取第 1 和第 2 个以及剩余的元素，前两个元素分别赋值给变量 i 和 j，
# 剩余的元素（是一个列表）赋值给变量 k，此时变量 k == [3, 4, 5]
让 [i, j, ...k] = a
```

需注意的是**剩余**关键字 `...` （即三个点，同 "展开" 关键字）只能出现在中括号的末尾，诸如 `let [i, ...j, k]` 语句是有语法错误的。

### 映射表的解构

示例：

```js
让 名称 = {"姓": "张", "名": "三", "辈": "伯"};

# a == "张", b == "三"
let {"姓": a, "名": b} = 名称
```

```js
let name = {"firstName": "foo", "lastName": "bar", "middleName": "D"};

# a == "foo", b == "bar"
let {"firstName": a, "lastName": b} = name
```

如果映射表的 key 是字符串，也可以省略其双引号，这样会得到跟 key 名称一样的变量。

示例：

```js
# 姓 == "张", 名 == "三"
让 {姓, 名} = name
```

```js
# firstName == "foo", lastName == "bar"
let {firstName, lastName} = name
```



### 元组的解构

示例：

```js
让 v = (123, "foo", true)

# a == 123
让 (a) = v

# a == 123, b == "foo", c == true
让 (a, b, c) = v

# a == true
让 (_, _, a) = v
```

```js
let v = (123, "foo", true)

# a == 123
let (a) = v

# a == 123, b == "foo", c == true
let (a, b, c) = v

# a == true
let (_, _, a) = v
```

在上例中，其中的 `_` 符号（下划线）表示仅匹配位置，丢弃其值。

嵌套的元组也能解构，示例：

```js
let t = ("foo", ("abc","xyz"), "bar")

# a == "abc", b == "xyz"
let (_,(a, b),_) = t
```

### 结构体的解构

结构体的解构必须使用其**默认构造函数**的各成员的顺序获取各个成员的值。

示例：

```js
让 u = 用户(1, "foo", 99)

# id == 1
让 User(id) = u

# id == 1, name == "foo"
让 User(id, name) = u

# id == 1, name == "foo", score == 99
让 User(id, name, score) = u

# score == 99
让 User(_, _, score) = u
```

```js
let u = User(1, "foo", 99)

# id == 1
let User(id) = u

# id == 1, name == "foo"
let User(id, name) = u

# id == 1, name == "foo", score == 99
let User(id, name, score) = u

# score == 99
let User(_, _, score) = u
```

在上例中，其中的 `_` 符号（下划线）表示仅匹配位置，丢弃其值。

联合体的结构体类型成员的解构方法跟结构体的解构一样。

注意，结构体的解构也可以按成员的名称来解构（假如成员有名称的话），示例：

```js
让 User(id=a, name=b) = u
```

当一个结构体的成员数量比较多，且只需很少的部分成员的值时，按成员名称来解构（而不是按参数顺序来解构）能简便很多。

#### 结构体匹配(::内容跟上一段重复，待编辑)

结构体的匹配表达式一般为：按照该结构体的默认构造函数的成员的出现顺序，列出字面量、常量或者变量的组合，比如上例中的 `case Book(title, isbn)` 就是将 `Book` 的两个成员按顺序列出。

另外一种格式是，按照成员的名称列出字面量、常量或者变量，示例：

```js
match v
    case Book(title="foo", isbn="123"):
        ...
    case Book(title="bar", isbn=x):
        ...
end
```

跟调用普通函数的情况类似，可以混合按参数顺序和按参数名称书写结构体的匹配表达式，但必须先写完所有按顺序的参数，才可以开始写按名称的参数。

如果匹配表达式里的成员名称和变量名一样，也可以省略书写成员名称。

`case Book(title, isbn)`

等同于：

`case Book(title=title, isbn=isbn)`

#### 联合体匹配

在匹配一个模式表达式时，运行环境在编译截断会事先检查被匹配的数据和匹配表达式的数据类型是否一致，如果不一致则不通过编译。

但联合体因为可以有多个成员（为常量或者结构体），所以在运行时，首先会对子类型进行匹配，类型一致之后再对值进行匹配。

比如有一个联合体：

```js
union Work
    Book(String title, String isbn)
    Album(String title, String artist)
end
```

模式匹配语句如下：

```js
01  match v
02      case Book(title, isbn):
03          writeLineFormat ("Book title: {}, ISBN: {}", title, isbn)
04      case Album(title, artist):
05          writeLineFormat ("Album title: {}, artist: {}", title, artist)
06  end
```

当变量 `v` 的值为 `Book` 的实例，02 行会被匹配中，如果值为 `Album` 的实例，则 04 行会被匹配中。



#### 部分匹配

对于一个复合结构的数据（比如列表、映射表、结构体、联合体的成员），模式表达式中的对结构的要求是不可反驳的，即 "实际数据的结构和组成" 跟 "模式匹配表达式要求的" 必须严格地一一对应。

也就是说匹配一个复合结构的数据时，表达式必须把全部成员和元素都列出才能匹配成功。如果不想全员列出（比如有时只需要当中的部分数据），可以使用 “...” 符号（三个点号）表示省略余下成员或元素，即此部分匹配是可选的（换句话说，也就是能匹配 0 或者多项）。比如：

* `case [first, second, ...remains]`
  表示匹配具有两个或两个以上元素的列表，且从第三个开始的元素的值存储在 `remains` 列表中，这个列表有可能是空列表。

* `case {id, name, ...remains}`
  表示匹配具有 "id" 和 "name" 两个 key 的映射表，且把其他 key 及值存储在 `remains` 映射表中，这个映射表有可能是空的。

* `case User(id, name, ...remains)`
  表示匹配数据类型为 `User`，且具有 "id" 和 "name" 两个成员的结构体（或者联合体当中名称为 `User` 的结构体类型成员），结构体的其他成员的值会被存储到 `remains` 元组中，这个元组有可能是空元组。

* `case User(id = x, name = y, ...remains)`
  表示匹配数据类型为 `User`，且具有 "id" 和 "name" 两个成员的结构体，这两个成员的值分别存储在变量 `x` 和 `y`，结构体的其他成员的值会被存储到 `remains` 元组中，这个元组有可能是空元组。

* `case (one, two, ...remains)`
  表示匹配具有两个或两个以上成员的元组，且从第三个开始的成员的值存储在 `remains` 元组中，这个元组有可能是空元组。

注意，如果对其余的数据不感兴趣（即上面示例当中的 `remains` 变量所存储的数据），也可以把 `remains` 变量名更改为 `_`（下划线符号），即 `..._` 表示丢弃其匹配所得的数据。为了简洁起见，甚至可以把 `_` 符号也省略，只保留 `...` 符号即可。比如 `case [first, second, ...]`。

#### 带条件的模式匹配

可以在模式匹配表达式后面加上 `如果` 条件语句，用于提供额外的匹配条件（也叫 "守卫" "Guard"），

(:: 考虑使用 `要求`（`require`）关键字替换 `如果` 关键字）

示例：

```js
匹配 v
    情况 (x,y,z) 如果 x > 10: # 仅当后面的条件表达式返回 `真` 时该分支才被匹配中。
       ...
以上
```

```js
match v
   case (x,y,z) if x > 10:
       ...
end
```

`如果` 条件语句能够使用其作用域已存在的变量，包括写在匹配表达式里的变量。

#### 带解析的模式匹配

有时用于 "被匹配的数据" 并不是最终所需的数据，可能需要经过一定的转换后才是所需的数据。模式匹配支持同时 "转换" 和 "匹配"。

示例：

假设原始数据是 String 类型，既可以解析为 `Email(name, domain)` 类型，也能解析为 `Phone(countryCode, number)`，`Email` 和 `Phone` 是联合体 `SocialId` 的两个成员：

```js
联合体 社交帐号
    电子邮箱(字符串 name, 字符串 domain)
    电话(字符串 countryCode, 字符串 number)
end

实现 解析器<电子邮箱, 字符串> 到 电子邮箱
    函数 可选<电子邮箱> 解析(字符串 s)
        匹配 正则("^(.+)@(.+)$").查找(s)
            情况 有([_, name, domain]): 有(电子邮箱(name, domain))
            情况 无: 无
        以上
    以上
以上

实现 解析器<电话, 字符串> 到 电话
    函数 可选<电话> 解析(字符串 s)
        匹配 正则("^(\\+\\d+)-(\\d+)$").查找(s)
            情况 有([_, countryCode, number]): 有(电话(countryCode, number))
            情况 无: 无
        以上
    以上
以上
```

```js
Union SocialId
    Email(String name, String domain)
    Phone(String countryCode, String number)
end

implement Parser<Email, String> to Email
    function Option<Email> parse(String s)
        match Regex("^(.+)@(.+)$").find(s)
            case Some([_, name, domain]): Some(Email(name, domain))
            case None: None
        end
    end
end

implement Parser<Phone, String> to Phone
    function Option<Phone> parse(String s)
        match Regex("^(\\+\\d+)-(\\d+)$").find(s)
            case Some([_, countryCode, number]): Some(Phone(countryCode, number))
            case None: None
        end
    end
end
```

下面是测试代码：

```js
# a == None
let a = Parser<Email, String>.parse("foobar")

# b == Some(Email("foo", "bar"))
let b = Parser<Email, String>.parse("foo@bar")

# c == None
let c = Parser<Phone, String>.parse("123")

# d == Some(Phone("+86", "123456"))
let d = Parser<Phone, String>.parse("+86-123456")
```

如果需要在 `匹配` 语句当中先对数据进行解析再匹配，则需要模式匹配表达式之前（或者说，在 `情况` 关键字之后）加上 `解析` 关键字。

示例：

```js
让 s = "foo@bar"

# 注意变量 's' 是字符串类型，而
# 模式匹配表达式的分别是 Email 和 Phone 类型

匹配 s
    情况 解析 Email(name, domain):
        书写行("一个电子邮箱")
    情况 解析 Phone(countryCode, number):
        书写行("一个电话号码")
    默认:
        书写行("未检测到")
以上
```

```js
let s = "foo@bar"

# Note that the variable 's' is a String, while
# the data type in the matching pattern expression are Email and Phone.

match s
    case parse Email(name, domain):
        writeLine("It's an Email address")
    case parse Phone(countryCode, number):
        writeLine("It's a phone number")
    default:
        writeLine("Not detected")
end
```

如果模式匹配发生在函数的参数，则 `解析` 关键字加在模式表达式之前，比如：

```js
模式函数 测试 (解析 电子邮箱(name, domain), 解析 电话(countryCode, number))
    ...
以上
```

```js
pattern function test (parse Email(name, domain), parse Phone(countryCode, number))
    ...
end
```

#### 正则表达式模式匹配

正则函数 `正则(expression).查找(String)`（`Regex(expression).find(String)`） 成功时返回的是一个被 `可选`（`Option`）联合体的成员 `有`(`Some`) 封装的列表，列表的第一个元素是匹配中的内容（字符串），从第二个元素开始是各个匹配组的值。find 函数失败时返回的是 `无`（`None`）。

示例：

假设一个简单的 Email 地址正则表达式为 "^(.+)@(.+)$"：

```js
让 s = "foo@domain"
让 ss = 正则("^(.+)@(.+)$").查找(s)
```

则 `ss` 的值为 `Some(["foo@domain", "foo", "domain"])`。

可以把正则函数 `查找` 的结果结合模式匹配：

```js
匹配 ss
    情况 有([_, name, domain]):
        输出格式行("名称是: {}, 域名是: {}", name, domain)
    默认:
        书写行("未侦测到电子邮箱")
以上
```

```js
match ss
    case Some([_, name, domain]):
        writeLineFormat("Name is: {}, domain is: {}", name, domain)
    default:
        writeLine("No Email address detected")
end
```

因为这种匹配情况比较常见，所以模式匹配支持同时 "正则匹配" 和 "模式匹配"，跟 `解析` 关键字类似，只需把模式表达式换成正则表达式，然后在表达式前面加上 `正则匹配` 关键字。

示例：

```js
让 s = "foo@domain"

匹配 s
    情况 正则匹配 /^(.+)@(.+)$/ [email, name, domain]:
        输出格式行("是一个电子邮箱: {}", email)
    情况 正则匹配 /^(\\+\\d+)-(\\d+)$/ [phone, countryCode, number]:
        输出格式行("是一个电话号码: {}", phone)
    默认:
        书写行("未侦测到")
以上
```

```js
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

需要注意的是，使用正则匹配的模式匹配时，"被检查的数据" 必须是字符串类型。

<!-- 如果需要指定正则匹配的参数，则使用 `(pattern_expression, option_value_or_list)` 元组代替正则表达式字符串。-->

`regular` 后面也能接受一个正则实例，使用正则构造函数 `Regex::new(String, Options)` 或者使用正则字面量 `/String/` 均可构建正则对象。

示例：

```js
match s
    case regular Regex::new("[a-z]+", RegularConst.ignoreCase) [name]:
        ...
    case regular /[0-9]+/ [number]:
        ...
end
```

#### 模板字符串模式匹配

模板字符串模式匹配是正则表达式匹配的简化版。

示例：

```js
match s
    case template `/user/{userName:\w+}`:
        writeLineFormat("Get user {}", userId)
    case template `/user/{userName:\w+}/post/{postId:\d+}`:
        writeLineFormat("Get post {}", postId)
end
```

其中的 `template ...` 会被解析为 `regular ...`，而模板字符串里面的占位符 `{...}` 是正则表达式以及其捕获值所存储的变量名，如果省略正则表达式部分，默认正则表达式是 `(.+)`。上面的代码会被解析为：

```js
match s
    case regular /\/user\/(\w+)/ [userName]:
        ...
    case regular /\/user\/(\w+)\/post\/(\d+)/ [userName, postId]:
        ...
end
```

#### 匹配复合结构数据时保留原始值

当匹配一个复合结构的数据时，写在匹配表达式里的变量获取的是原始数据的某个部分，如果想获取完整的原始数据，可以在模式表达式之前加上一个变量名称加上 `展开`（`expand`）关键字。

示例：

```js
让 v = 通过ID获取用户(123)
匹配 v
    情况 u 展开 用户(id, name):
        输出格式行("id: {}, 名称: {}", id, name)
        输出格式行("{:?}", u)
以上
```

```js
let v = getUserById(123)
match v
    case u expand User(id, name):
        writeLineFormat("id: {}, name: {}", id, name)
        writeLineFormat("{:?}", u)
end
```

如果模式匹配发生在函数的参数，则 `展开` 关键字加在模式表达式之前，比如：

```js
函数 测试 (u 展开 User(id, name))
    ...
以上
```

```js
function test (u expand User(id, name))
    ...
end
```

如果 `解析` 和 `展开` 同时进行，则先写 `展开` 再写 `解析` 关键字，比如：

```js
匹配 v
    情况 u 展开 解析 用户(id, name):
        ...
以上
```

```js
match v
    case u expand parse User(id, name):
        ...
end
```

```js
函数 测试 (u 展开 解析 User(id, name))
    ...
以上
```

```js
function test (u expand parse User(id, name))
    ...
end
```

### 匹配一个数字范围

```js
match i
    case range 1..2:
        ...
```

```js
match c
    case range 'a'..'f':
        ...
```

### 嵌套匹配

可以匹配多层次数据：

```js
match a
    case User(name, addr: {city, street}, id):
        ...
```

`expand, range, regex` 等关键字可以用于嵌套内的变量值。

```js
match a
    case User(name, score: v expand range 60..100):
        # got `name` and `v`
    case User(name: n regex /^foo/, score):
        # got `n` and `score`
```


### `如果 让...匹配` 语句

有时可能仅仅为了匹配一种模式，这时可以使用 `如果 让...匹配`（`if let...match`） 语句，而无必要使用完整的 `匹配` 语句。

示例：

```
让 v = (123, 456)
如果 让 (a, b) 匹配 v 那么
    输出格式行 ("a 是: {}, b 是: {}", a, b)
以上

如果 让 (123, b) 匹配 v 那么
    输出格式行 ("a 是 123")
以上
```

```
let v = (123, 456)
if let (a, b) match v then
    writeLineFormat ("a is: {}, b is: {}", a, b)
end

if let (123, b) match v then
    writeLineFormat ("a is 123")
end
```

在 `让` 关键字和 `匹配` 关键字之间除了可以是一个模式匹配表达式，也可以加入上面 `匹配` 语句当中提到的 `解析`、`展开` 和 `正则匹配` 等关键字。示例：

```js
如果 让 解析 User(id, name) 匹配 v 那么 ...
如果 让 u 展开 解析 User(id, name) 匹配 v 那么 ...

如果 让 正则匹配 "^(.+)@(.+)$" [email, name, domain] 匹配 v 那么 ...
如果 让 u 展开 正则匹配 "^(.+)@(.+)$" [email, name, domain] 匹配 v 那么 ...
```

`让...匹配` 表达式返回的是一个 `逻辑`（`Boolean`）类型的数值，所以有时还可以跟其他条件一起组合成更为复杂的条件语句。比如：

```js
if let (id, name) match user1 :and id > 100 then
    ...
end
```

> `让...匹配` 表达式不能单独写成一条语句，因为这样很容易因为忘记判断其返回值而使用模式匹配表达式里的变量值，所以语法上规定  `让...匹配` 表达式只能写在 `如果` 语句、`分支` 语句、`条件` 语句里。

### 模式匹配函数 (NEW)

如果有一组函数的签名完全一样，即参数列表的参数数量和类型和顺序都一样，且这些函数 <!--加上了标注 `@模式`（`@pattern`）--> 定义语句前添加了 `模式`（`pattern`）关键字，则这组函数被称为 `模式匹配函数`。

普通函数（相对于模式函数来说）的每个参数只能是 **单独的一个变量**，而模式函数在定义参数时，就可以写上模式匹配表达式，通常用这个方法来在接收参数的同时 "解构" 其中的值（比如解构一个结构体、解构一个元组等）。

示例：

```js
pattern function check(List<Int> [])
    writeLine("empty")
end

pattern function check(List<Int> [1,2])
    writeLine("there are two expected elements")
end

pattern function check(List<Int> [a,b])
    writeLine(`there are two elements {a} and {b}`)
end

pattern function check(List<Int> list)
    writeLine("a list")
end
```

模式函数会被运行环境自动解析为带有模式匹配的分支函数，即上面的代码等同于：

```js
function check(List<Int> p) branch
    case let [] match p:
        begin
            writeLine("empty")
        end
    case let [1, 2] match p:
        begin
            writeLine("there are two expected elements")
        end
    case let [a, b] match p:
        begin
            writeLine(`there are two elements {a} and {b}`)
        end
    case let list match p:
        begin
            writeLine("a list")
        end
    end
end
```

需要注意，模式匹配函数的各子函数（第一个子函数可以省略）上面的 `@模式`（`@pattern`） 标注是必须的，否则运行环境会认为你重复定义了函数，并且会引起运行时错误以阻止运行。

当然模式匹配函数的各子函数里仍然能够使用条件分支，并且在该子函数的条件分支都不满足时，会自动跳到下一个个体，而不是（像分支函数那样）直接出错。也就是说在模式匹配函数里的子函数里的分支是该分支的一道防线（也叫函数守卫、Guard）。

(::TODO)
（::考虑在 pattern 函数里不再支持分支，而是增加 `要求`（`require`）关键字。）

示例：

```js
function check(List<Int> [])
    writeLine("empty")
end

@pattern
function check(List<Int> [a,b]) branch
    case a > b:
        writeLine("a > b")
    case a < b:
        writeLine("a < b")
end

@pattern
function check(List<Int> list)
    writeLine("a list")
end
```

会被运行环境解析为：

```js
function check(List<Int> p) branch
    case let [] match p:
        begin
            writeLine("empty")
        end
    case let [a, b] match p and passed where
        begin
            let (passed, value) = condition
                case a > b:
                    let r = begin
                        writeLine("a > b")
                    end
                    (true, r)
                case a < b:
                    let r = begin
                        writeLine("a < b")
                    end
                    (true, r)
                end
        end:
        value
    case let list match p:
        begin
            writeLine("a list")
        end
    end
end
```

注意如果在某个子函数的分支里存在 `默认`（`default`） 语句块，则显然一旦该子函数被匹配中，就不会因为分支条件不满足而跳到下一个子函数（因为 `默认` 语句块无条件接受了所有条件）。

示例：

```js
function check(List<Int> [])
    writeLine("empty")
end

function check(List<Int> [a,b]) branch
    case a > b:
        writeLine("a > b")
    case a < b:
        writeLine("a < b")
    default:
        writeLine("equals")
end

function check(List<Int> list)
    writeLine("a list")
end
```

会被运行环境解析为：

```js
function check(List<Int> p) branch
    case let [] match p:
        begin
            writeLine("empty")
        end
    case let [a, b] match p:
        begin
            condition
                case a > b:
                    begin
                        writeLine("a > b")
                    end
                case a < b:
                    begin
                        writeLine("a < b")
                    end
                case a < b:
                    begin
                        writeLine("equals")
                    end
            end
        end
    case let list match p:
        begin
            writeLine("a list")
        end
    end
end
```
