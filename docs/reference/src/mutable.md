# 可变性

e.g.

window.title := "hello"
box.width := 120
frame.rect.left := 10

运算符 `:=` 是函数 `!set` 的语法糖：

!set(window, #title, "hello") <-- 返回 window
!set(box, #width, 120) <-- 返回 box
!set(get(frame, #rect), #left, 10) <-- 返回 rect

这些对象的成员并不是普通的结构体成员，所以读取的方法跟其他普通结构体实例的不一样：

let String title := window.title
let Int width := box.width
let Int left := frame.rect.left

表达式 `let ... :=` 是函数 `!get` 的语法糖：

let String title = !get(window, #title)
let Int width = !get(box, #width)
let Int left = !get(get(frame, #rect), #left)

## 内置的对象 prop

prop 即属性包，每个线程都会有一个独立的属性包

属性包也可以用 `:=` 操作符读写：

prop.a := 12
let Int a := prop.a

!set(prop, #a, 12)
let Int a = !get(prop, #a)

prop.set(#a, 12)
let Int a = prop.get(#a)

prop 不是一个普通对象，它会作为一个特殊关键字被语言限制，它本身无法作为一个值传递给其他变量或者作为参数传递给函数。
