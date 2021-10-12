# 模式匹配和模式解构

## 模式匹配

模式匹配用于 `匹配`（`match`） 语句、函数的参数传值、以及 `如果 让...匹配...` 语句，这 3 种场合。

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

#### 模式表达式

模式表达式由字面量（包括列表的中括号、元组的括号、映射表的花括号）、常量及变量等组成。

其中字面量和常量用于跟 "待检查的数据" 作相等比较，而变量则作为占位符，当匹配成功时（即字面量和常量相等、结构相同、元素或成员个数相同），变量则会捕获其所对应的数值。

需注意，模式表达式当中的变量：

* 如果匹配语句所在的作用域当中已存在（且比匹配语句更早出现）同名的变量名，则模式表达式当中的变量会覆盖外面的变量，新变量的作用域包括该分支的代码。

示例：

  ```js
  让 i = 123
  让 v = (77,88)
  匹配 v
      情况 (i, j):
          输出行 (i) // 输出 '77' 而不是 '123'
  以上
  ```

  ```js
  let i = 123
  let v = (77,88)
  match v
      case (i, j):
          writeLine (i) // output '77' instead of '123'
  end
  ```

* 如有匹配式中存在两个或以上同名变量，则第一个会被赋值，第二个及之后的会当成常量来比较。示例：

  ```js
  让 v = (11, 22, 22, 11)
  匹配 v
      情况 (a, a, b, b):
          输出行 ("匹配失败")
      情况 (a, b, b, a):
          输出行 ("匹配成功，变量 a 的值将会是 11, b 是 22")
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

#### 类型匹配

在匹配一个模式表达式时，首先会对数据的数据类型进行匹配，数据类型一致之后再对值进行匹配。

比如有一个联合体：

```js
union Art
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

对于一个复合结构的数据（比如列表、映射表、结构体、联合体的子类型、元组），模式表达式中的对结构的要求是不可反驳的，即 "实际数据的结构和组成" 跟 "模式匹配表达式要求的" 必须严格地一一对应。

也就是说匹配一个复合结构的数据时，表达式必须把全部成员和元素都列出才能匹配成功。如果不想全员列出（比如有时只需要当中的部分数据），可以使用 “...” 符号（三个点号）表示省略余下成员或元素的列举，比如：

* `case [first, second, ...remains]`
  表示匹配具有两个及两个以上元素的列表，且从第三个开始的元素的值存储在 `remains` 列表中，这个列表有可能是空列表。

* `case {id, name, ...remains}`
  表示匹配具有 "id" 和 "name" 两个 key 的映射表，且把其他 key 及值存储在 `remains` 映射表中，这个映射表有可能是空的。

* `case User(id, name, ...remains)`
  表示匹配数据类型为 `User`，且具有 "id" 和 "name" 两个成员的结构体（或者联合体当中名称为 `User` 的子类型），结构体的其他成员的值会被存储到 `remains` 元组中，这个元组有可能是空元组。

* `case (one, two, ...remains)`
  表示匹配具有两个及两个以上成员的元组，且从第三个开始的成员的值存储在 `remains` 元组中，这个元组有可能是空元组。

注意，如果对其余的数据不感兴趣（即上面示例当中的 `remains` 变量所存储的数据），也可以把 `remains` 变量名更改为 `_`（下划线符号），即 `..._` 表示丢弃其匹配所得的数据。为了简洁起见，甚至可以把 `_` 符号也省略，只保留 `...` 符号即可。比如 `case [first, second, ...]`。

#### 带条件的模式匹配

可以在模式匹配表达式后面加上 `如果` 条件语句，用于提供额外的匹配条件，示例：

```js
匹配 v
    情况 (x,y,z) 如果 x > 10: // 仅当后面的条件表达式返回 `真` 时该分支才被匹配中。
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

假设原始数据是 String 类型，既可以解析为 `Email(name, domain)` 类型，也能解析为 `Url(scheme, path)`，`Email` 和 `Url` 是联合体 `SocialId` 的两个子类型：

```js
联合体 SocialId
    Email(String name, String domain)
    Url(String scheme, String path)
end

实现 解析器<Email, String> 到 Email
    函数 Email 解析(String s)
        匹配 s.分隔('@')
            情况 有((name, domain)): 有(Email(name, domain))
            情况 无: 无
        以上
    以上
以上

实现 解析器<Url, String> 到 Url
    函数 Url 解析(String s)
        匹配 正则("^(\\w+)://(.+)$").查找(s)
            情况 有([_, scheme, path]): 有(Url(scheme, path))
            默认: 无
        以上
    以上
以上
```

```js
Union SocialId
    Email(String name, String domain)
    Url(String scheme, String path)
end

implement Parser<Email, String> for Email
    function Email parse(String s)
        match s.split('@')
            case Some((name, domain)): Some(Email(name, domain))
            case None: None
        end
    end
end

implement Parser<Url, String> for Url
    function Url parse(String s)
        match Regex("^(\\w+)://(.+)$").find(s)
            case Some([_, scheme, path]): Some(Url(scheme, path))
            default: None
        end
    end
end
```

下面是测试代码：

```js
// a == None
let a = Parser<Email, String>.parse("foobar")

// b == Some(Email("foo", "bar"))
let b = Parser<Email, String>.parse("foo@bar")

// c == None
let c = Parser<Url, String>.parse("foobar.domain/user/123")

// d == Some(Url("https", "foobar.domain/user/123"))
let d = Parser<Url, String>.parse("https://foobar.domain/user/123")
```

如果需要在 `匹配` 语句当中先对数据进行解析再匹配，则需要模式匹配表达式之前（或者说，在 `情况` 关键字之后）加上 `解析` 关键字。

示例：

```js
让 s = "foo@bar"

// 注意变量 's' 是字符串类型，而
// 模式匹配表达式的分别是 Email 和 Url 类型

匹配 s
    情况 解析 Email(name, domain):
        输出行("一个 Email 地址")
    情况 解析 Url(scheme, path):
        输出行("一个 Url 地址")
    默认:
        输出行("未检测到")
以上
```

```js
let s = "foo@bar"

// Note that the variable 's' is a String, while
// the data type in the matching pattern expression are Email and Url.

match s
    case parse Email(name, domain):
        writeLine("It's an Email address")
    case parse Url:
        writeLine("It's an Url")
    default:
        writeLine("Not detected")
end
```

如果模式匹配发生在函数的参数，则 `解析` 关键字加在模式表达式之前，比如：

```js
函数 测试 (解析 Email(name, domain), 解析 Url(scheme, path))
    输出格式行("Email 是: {}@{}", name, domain)
    输出格式行("Url 是: {}://{}", scheme, path)
以上
```

```js
function test (parse Email(name, domain), parse Url(scheme, path))
    writeLineFormat("Email is: {}@{}", name, domain)
    writeLineFormat("Url is: {}://{}", scheme, path)
end
```

#### 正则表达式模式匹配

正则函数 `正则(expression).查找(String)`（`Regex(expression).find(String)`） 成功时返回的是一个被 `可选`（`Option`）联合体的子类型 `有`(`Some`) 装箱的列表，列表的第一个元素是匹配中的内容（字符串），从第二个元素开始是各个匹配组的值。find 函数失败时返回的是 `无`（`None`）。

所以正则函数的 `查找` 方法的返回值可以跟模式匹配结合使用。

示例：

假设一个简单的 Email 地址正则表达式为 "^(.+)@(.+)$"：

```js
让 s = "foo@domain"
让 ss = 正则("^(.+)@(.+)$").查找(s)
```

则 `ss` 的值为 `Some(["foo@domain", "foo", "domain"])`，可以把正则函数 `查找` 的结果结合模式匹配：

```js
匹配 ss
    情况 有([_, name, domain]):
        输出格式行("名称是: {}, 域名是: {}", name, domain)
    默认:
        输出行("未侦测到 Email 地址")
以上
```

```js
match ss
    case Some([_, name, domain]):
        writeLineFormat("Name is: {}, domain is: {}", name, domain)
    default:
        writeLine("No Email detected")
end
```

因为这种匹配情况比较常见，所以模式匹配支持同时 "正则匹配" 和 "模式匹配"，跟 `解析` 关键字类似，只需把模式表达式换成正则表达式，然后在表达式前面加上 `正则匹配` 关键字。

示例：

```js
让 s = "foo@domain"

匹配 s
    情况 正则匹配 "^(.+)@(.+)$" [email, name, domain]:
        输出格式行("是一个 Email: {}", email)
    情况 正则匹配 "^(\\w+)://(.+)$" [url, scheme, path]:
        输出格式行("是一个 Url: {}", url)
    默认:
        输出行("未侦测到 Email 或 Url")
以上
```

```js
let s = "foo@domain"

match s
    case regular "^(.+)@(.+)$" [email, name, domain]:
        writeLineFormat("It's Email: {}", email)
    case regular "^(\\w+)://(.+)$" [url, scheme, path]:
        writeLineFormat("It's Url: {}", url)
    default:
        writeLine("Email or Url not detected")
end
```

需要注意的是，使用正则匹配的模式匹配时，"被检查的数据" 必须是字符串类型。

如果需要指定正则匹配的参数，则使用 `(pattern_expression, option_value_or_list)` 元组代替正则表达式字符串。示例：

```js
match s
    case regular ("[a-z]+", RegularConst.ignoreCase) [name]:
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

### `如果 让...匹配` 语句

有时可能仅仅为了匹配一种模式，这时可以使用 `如果 让...匹配`（`if let...match`） 语句，而无必要使用 "完整" 的 `匹配` 语句。

示例：

```
让 v = (123, 456)
如果 让 (a, b) 匹配 v 那么
    输出格式行 ("a 是: {}, b 是: {}", a, b)
以上
```

```
let v = (123, 456)
if let (a, b) match v then
    writeLineFormat ("a is: {}, b is: {}", a, b)
end
```

在 `让` 关键字和 `匹配` 关键字之间除了可以是一个模式匹配表达式，也可以加入上面 `匹配` 语句当中提到的 `解析`、`展开` 和 `正则匹配` 等关键字。示例：

```js
如果 让 解析 User(id, name) 匹配 v 那么 ...
如果 让 u 展开 解析 User(id, name) 匹配 v 那么 ...

如果 让 正则匹配 "^(.+)@(.+)$" [email, name, domain] 匹配 v 那么 ...
如果 让 u 展开 正则匹配 "^(.+)@(.+)$" [email, name, domain] 匹配 v 那么 ...
```

## 模式解构

模式解构即部分模式匹配，或者说非严格的模式匹配。模式解构的主要目的是为了获取复合结构数据的部分数据。

模式解构发生在 `让`（`let`）赋值语句、`遍历`（`iter`）语句的 `到`（`to`）关键字之后、`现有 让`（`for let`）循环语句的值初始化以及 `重复`（`loop`） 关键字之后，等 3 种场合。

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
现有 让 (a,b) = v1 如果 a>0 && b>0 那么
    ...
    让 i = ...
    让 j = ...
    重复 (i,j)
以上
```

XiaoXuan 的赋值语句实质是模式解构。需要注意的是，虽然模式解构只需部分匹配即可，但因为缺少 `匹配` 语句的 `默认` 分支，如果指定的部分不匹配，则会直接引起运行时异常。也就是说，对于赋值语句 `让`（`let`），除了声明的数据类型不匹配会引起运行时异常，模式不匹配也会引起。

示例：

```js
让 (a, b, b, a) = (1, 2, 3, 4)
```

上面的赋值语句（模式解构）虽然左右两边的数据类型一致（都是元组），但因为模式不匹配，所以会引起运行时异常。

### 列表的解构

示例：

```js
// 第一个 == 1, 第二个 == 2, 剩余 == [3,4,5,6]
让 [第一个, 第二个, ...剩余] = [1,2,3,4,5,6]

// 丢弃第一个和第二个元素的值， 第三个 == 3
让 [_, _, 第三个] = [1,2,3,4,5]

// 解构一个二维列表
// 第一个 == 1, 第二个 == 2, 第三个 == 3
让 [[第一个, 第二个], [第三个, _]] = [[1,2],[3,4],[5,6]]

// 使用索引来解构（未支持）
// 注意索引从 1 开始，而不是从 0 开始
// x == 1, y == 6
让 [1:x, 6:y] = [1,2,3,4,5,6]
```

```js
// first == 1, second == 2, rest == [3,4,5,6]
let [first, second, ...rest] = [1,2,3,4,5,6]

// drop the first and the second element value, third == 3
let [_, _, third] = [1,2,3,4,5]

// deconstructing a two-dimensional list
// first == 1, second ==2, third == 3
let [[first, second], [third, _]] = [[1,2],[3,4],[5,6]]

// Deconstructing using indexes (not supported)
// Note that indexes start at 1, not 0
// x == 1, y == 6
let [1:x, 6:y] = [1,2,3,4,5,6]
```

在上例中：

* 其中的 `...` 符号（三个点号）表示获取列表当中剩余的其他元素；
* 其中的 `_` 符号（下划线）表示仅匹配位置，丢弃其值。

## 映射表的解构

示例：

```js
让 名称 = {"姓": "张", "名": "三", "辈": "伯"};

// a == "张", b == "三"
let {"姓": a, "名": b} = 名称
```

```js
let name = {"firstName": "foo", "lastName": "bar", "middleName": "D"};

// a == "foo", b == "bar"
let {"firstName": a, "lastName": b} = name
```

如果映射表的 key 是字符串，也可以省略其双引号，这样会得到跟 key 名称一样的变量。

示例：

```js
// 姓 == "张", 名 == "三"
让 {姓, 名} = name
```

```js
// firstName == "foo", lastName == "bar"
let {firstName, lastName} = name
```

## 结构体的解构

结构体的解构必须使用其**默认构造函数**的各成员的顺序获取各个成员的值。

示例：

```js
让 u = 用户(1, "foo", 99)

// id == 1
让 User(id) = u

// id == 1, name == "foo"
让 User(id, name) = u

// id == 1, name == "foo", score == 99
让 User(id, name, score) = u

// score == 99
让 User(_, _, score) = u
```

```js
let u = User(1, "foo", 99)

// id == 1
let User(id) = u

// id == 1, name == "foo"
let User(id, name) = u

// id == 1, name == "foo", score == 99
let User(id, name, score) = u

// score == 99
let User(_, _, score) = u
```

在上例中，其中的 `_` 符号（下划线）表示仅匹配位置，丢弃其值。

联合体的子类型的解构方法跟结构体的解构一样。

## 元组的解构

示例：

```js
让 v = (123, "foo", true)

// a == 123
让 (a) = v

// a == 123, b == "foo", c == true
让 (a, b, c) = v

// a == true
让 (_, _, a) = v
```

```js
let v = (123, "foo", true)

// a == 123
let (a) = v

// a == 123, b == "foo", c == true
let (a, b, c) = v

// a == true
let (_, _, a) = v
```

在上例中，其中的 `_` 符号（下划线）表示仅匹配位置，丢弃其值。

嵌套的元组也能解构，示例：

```js
let t = ("foo", ("abc","xyz"), "bar")

// a == "abc", b == "xyz"
let (_,(a, b),_) = t
```
