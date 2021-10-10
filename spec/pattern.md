# 模式匹配和模式解构

## 模式匹配

模式匹配用于 `匹配`（`match`） 语句、函数的参数匹配、以及 `如果...匹配...` 语句。

### `匹配`（`match`）语句

模式匹配会对 "待检查的数据" 的数据类型、结构、值等进行比较，只有完全匹配的分支才被执行。

示例：

```
让 v = ("foo","bar","foo")
让 r = 匹配 v
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

```
let v = ("foo","bar","foo")
let r = match v
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

模式表达式由字面量（包括列表的中括号、元组的括号、映射表的花括号）、常量及变量组成。其中字面量和常量用于跟 "待检查的数据" 作相等比较，而变量则作为占位符号，当模式匹配时（即字面量和常量相等、结构相同、元素或成员个数相同），变量则捕获所对应的数值。

模式表达式当中的变量需注意：

* 如果匹配语句所在的作用域当中已存在（且比匹配语句早出现）同名的变量名，则模式表达式当中的变量会覆盖外面的变量，包括该分支的代码，访问该变量是其值是模式表达式里变量的值，而不是外面同名变量的值。示例：

  ```
  让 i = 123
  让 v = (77,88)
  匹配 v
      情况 (i, j):
          输出行 (i) // 输出 '77' 而不是 '123'
  以上
  ```

  ```
  let i = 123
  let v = (77,88)
  match v
      case (i, j):
          writeLine (i) // output '77' instead of '123'
  end
  ```

* 如有匹配式中存在两个或以上同名变量，则第一个会被赋值，第二个及之后的会当成常量来比较。示例：

  ```
  让 v = (11, 22, 22, 11)
  匹配 v
      情况 (a, a, b, b):
          输出行 ("匹配失败")
      情况 (a, b, b, a):
          输出行 ("匹配成功，变量 a 的值将会是 11, b 是 22")
  以上
  ```

  ```
  let v = (11, 22, 22, 11)
  match v
      case (a, a, b, b):
          writeLine ("Failed")
      case (a, b, b, a):
          writeLine ("Ok, the value of 'a' will be 11, 'b' is 22")
  end
  ```

#### 类型匹配

在匹配一个模式表达式时，首先会进行数据