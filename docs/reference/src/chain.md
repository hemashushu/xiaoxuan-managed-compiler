# 函数的连续调用

<!--
## 一般函数的连续调用

### 类型方法

:TODO

### 参数传参（::TODO 不再支持）

数据作为函数的最后一个参数传递

"hello" -> writeLine()
-->

将一个数值（或者一个表达式的求值结果）作为下一个表达式的参数，如：


`3 >> add(%, 2)`

等同于

`add(3, 2)`

此方法可以将一串处理函数连接起来，就像管道一样

```js
let result = tokenlize("1+2*3") >>
    parse(%) >>
    eval(%)
// result 的结果为 7
```

也可以一次将多个数值传递下去

`(2, 3) >> add(%, %)`

等同于

`add(2, 3)`

## 方法的连续调用

即点号的连续调用，一个方法的返回值是一个数值，该数值也存在方法，可以一路点号调用。

```
let a = [1..10]
    .map(i=>i*3)
    .filter(i=>(i :rem: 2) == 0)
```

## Option, Result 类型的连续调用

::todo 重新考虑运算符号

~|, ~&

check_one(1) ~| check_two(2)
check_one(1) ~& check_two(2)


Option, Result 此类数据扮演着 Functor 部分角色，但为了避免引入深奥理论，这里命名为 Container。

`>>` 串联符号能将 "单独一个值参数和一个 Container 类型返回值" 的函数串联起来。

let f = Option <- (T)

f1 >> f2 >> f3 >> f4

只要有一个环节的值为 Nothing，则链路直接跳到最后一环并输出 Nothing，否则返回 Some(value)。
只要有一个环节的值为 Err，则链路直接跳到最后一环并输出 Err，否则返回 Ok(value)。

### 对于返回非 Option 的函数，可以使用匿名函数包装然后放进链路

function String sample(Int i) = ...

let f2b = v => new Some(sample(v))
let f2b = v => Some::new(sample(v))

然后就可以加入到链路了

f1 >> f2 >> f2b >> f3 >> f4

也可以在链路上当场转换：
f1 >> f2 >> v => Some::new(sample(v)) >> f3 >> f4

或者使用函数合并操作符：
f1 >> f2 >> Some::new & sample >> f3 >> f4

### 有时需要在中间一环提取部分数据，而不是直接把上一环的数据传到下一环，这时有两种方法：
1. 断链，即，将程序分成两段来写；
2. 将剩下的链放在一个匿名函数里。

示例：

f1 >> f2 >>
    b => begin
        let (i, j) = b
        Ok::new(i) >> f3 >> f4
    end

### 两个或多个参数的情况

将 f1 的结果同时应用到 f2, f3 的参数
f1 >> (f2, f3)

将 f1, f2 的结果作为两个参数应用到 f3
(f1, f2) >> f3

连续调用，f4 接受两个参数 f2, f3
f1 >> (f2, f3) >> f4

连续调用，f4 接受一个 Point(x,y) 参数
f1 >> (f2, f3) >> (i, j) => Ok::new(Point::new(i, j)) >> f4

连续调用，丢弃中间的一个结果值，f4 接受一个参数
f1 >> (f2, f3) >> (i, _) => Ok::new(i) >> f4

## Option, Result 类型的短路拆封运算

a() || b() || c() || d

只要有一个返回非 Option::None 和 Result::Err 即立即返回其数值。

:TODO

## 管道调用

用于创建绿色线程。

a() | b() | c()

:TODO