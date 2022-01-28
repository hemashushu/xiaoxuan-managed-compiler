# 并行

## 线程变量

::TODO

core::variable

* void <- set<T>(HashString name, T value)
  更新

* Option<T> <- tryGet<T>(HashString name)
  尝试读取

* T <- get<T>(HashString name)
* void <- delete<T>(HashString name)

```js
无返回 <- 设值<T>(散列字符串 name, T value)
T <- 读值<T>(散列字符串 name)
可选<T> <- 尝试读值<T>(散列字符串 name)
无返回 <- 删除<T>(散列字符串 name)
```

线程变量的实现方式由具体的运行环境决定。一般使用一个内置的线程关联的映射表 `Map<HashString, T>` 实现。

示例：

`set(#foo, 123)`
`set(#bar, "hello")`

`let i = get<Int>(#foo)`
`let j = get<String>(#hello)`

`let Int i = get(#foo)`
`let String j = get(#hello)`

### 线程变量的数据类型

对于每种数据类型的线程变量的读写，都会产生相应的一个独立映射表，比如：

```js
set<Int>(#"foo", 123)
set<String>(#"foo", "bar")
```

上面两句会分别产生一个 `Map<HashString, Int>` 和一个 `Map<HashString, String>`，两个映射表分别存储相应数据类型的值，所以即使看起来线程变量可以接受任何数据类型，但实际上还是严格遵守静态数据类型检查。