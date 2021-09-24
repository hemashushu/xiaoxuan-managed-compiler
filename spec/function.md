# 函数

## 函数的调用

* 函数的调用格式：

  `函数名称 (参数值1, 参数值2, ... ,参数值N)`

  示例：

  `加 (1, 2)`
  `行输出 ("你好")`

* 调用函数并获取返回值：

  `让 变量名 = 函数名称 (参数值1, 参数值2, ..., 参数值N)`

  示例：

  `让 a = 平方根 (4)`
  `让 b = 加 (1, 2)`

* 支持按参数名称调用：

  `函数名称 (参数名称1=参数值1, 参数名称2=参数值2, ..., 参数名称N=参数值3)`

  也可以混合按位置和按名称两种传参数方式，不过必须先传完所有的位置参数才可以开始按名称传参数。

  `函数名称 (参数值1, ..., 参数值N, 参数名称a=参数值a, ..., 参数名称z=参数值z)`

  示例：

  `绘图.直线 (x1=0, y1=0, x2=100, y2=100, 颜色=颜色常数.红色)`
  `绘图.直线 (0, 0, 100, 100, 颜色=颜色常数.红色, 线条宽度=3)`

* 调用一个没有参数的函数时，需要在函数名称后面加上空元，即一对括号：

  `函数名称 ()`

* 函数也可以嵌套调用，即：将一个函数的返回值作为参数值传递给另一个函数。也可以是另一个值表达式，这时需要将值表达式用括号包围起来：

  `函数名称1 (参数1, 函数名称2 (参数1, 参数2), 任意值表达式N)`

  示例：

  `让 弦 = 平方根 (平方 (3), 平方 (4))`

  + 一般情况下，尽量先把表达式的值求出来（并赋值给变量），然后再传入函数的参数，避免多层函数嵌套调用。
  + 在语法上，允许省略函数名称和参数括号，以及参数之间的空格，不过为了规范起见尽量这些空格。

* 当参数列表过长时，可以换行书写，但函数名称后面的第一个括号必须保留在函数名称后面，示例：

```
00  让 标准差 =
01    平方根 (
02      加 (
03        (x1-μ)^2,
04        (x2-μ)^2
05      )
05    )
```

  注意上面示例的第 01 和 02 行的末尾的左括号 "(" 必须跟在函数名称后面。

## 函数的中置调用

如果一个函数只有 2 个参数，则在调用时可以把它写成中置格式，方法是使用反单引号将函数名包围起来，然后将 2 个参数分别写在函数的左右两边。比如函数：

```
让 a = 加 (11, 55)
```

可以写成：

```
让 a = 11 `加` 55
```

### 名称由纯符号组成的函数

所有名称由**纯符号**构成的函数使用中置格式调用。比如函数 "+" 和 "*" 在调用时应该使用中置格式，实际上它们是函数 "加" 和 "乘" 的别名。比如下面 3 句的作用完全相同：

```
00  让 a = 加 (11, 55)
01  让 b = 11 `加` 55
02  让 c = 11 + 55
```

### 优先级和结合性

当**多个中置函数连续调用**时，不同的中置函数可以有不同的优先级别（即执行的先后顺序），相同优先级别的又有 "从左到右" 和 "从右到左" 两种不同的结合方向。优先级和结合方向由定义函数时定义的。

默认情况下，所以中置函数的优先级别一样，且都遵循 "从左到右" 的结合方向。比如：

`让 a = 1 + 8 - 3`

在求值时，首先会执行表达式 `1 + 8`，得出值 `9` 之后再执行表达式 `9 - 3`，最后得出结果 `6`。再比如：

`让 b = 1 + 2 * 3`

因为 `*` 函数（对应着 "base.math.mul" 或者 "基本.数学.乘" 函数）定义的优先级别比 `+` 的要高，所以会先执行表达式 `2 * 3`，得出值 `6` 之后再执行表达式 `1 + 6`，最后得出结果为 `7`。

从右到左的结合方向的函数比较少，常见的有幂运算函数 `^`，比如：

`让 c = 4 ^ 3 ^ 2`

它会先执行表达式 `3 ^ 2`，得出值 `9` 之后再执行表达式 `4 ^ 9`。

## 函数的链式调用
