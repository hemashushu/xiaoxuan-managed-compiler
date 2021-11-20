# 并行

## 线程变量

* void <- setVar<T>(KeyString name, T value)
* Option<T> <- tryGetVar<T>(KeyString name)
* T <- getVar<T>(KeyString name)
* void <- removeVar(KeyString name)

```js
void <- writeMut(KeyString name, T value)
T <- readMut<T>(KeyString name)
Option<T> <- tryReadMut<T>(KeyString name)
void <- removeMut(KeyString name)
```

```js
无返回 <- 写变数(散列字符串 name, T value)
T <- 读变数<T>(散列字符串 name)
可选<T> <- 尝试读变数<T>(散列字符串 name)
无返回 <- 删除变数(散列字符串 name)
```

线程变量一般使用一个线程关联的内部映射表（键为字符串类型，值可为任意数据类型）实现，因为运行时会对字符串字面量会自动生成散列值，所以读写线程变量一般场合不会有性能问题。