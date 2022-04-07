# interface

`接口`（`interface`） 是一种约定所有方法必须遵循以下规定的 `特性`：

1. 第一个参数必须为 `本型`（`Self`），且这个本型必须与所处理的数据无关，即这个本型是一个占位符，跟业务逻辑无关；
2. 必须是空方法。

相对于 `特性` 用于实现具体数据类型的特有方法，`接口` 则用于实现某个（或某组）业务逻辑的方法。

示例：

```js
struct Ticket
    Int id
    String issue
end

interface TicketRepository
    empty function Result<Unit, Error> add(Self self, Int id, String issue)
    empty function Ticket get(Self self, Int id)
end
```

实际上 `接口` 在内部是一个被标上了额外的标记 `特性`，不过当需要表示业务逻辑的一组方法时，尽量使用 `接口` 而不是 `特性` 来定义，这样目的会更明确。

## 组件

实现接口跟实现一个特性类似，不同的是，`特性` 是实现在一个数据类型（比如结构体、联合体）上，接口则是实现在 `组件`（`component`） 上。组件里必须包含有 "实现该业务逻辑所需要的其他接口" 的成员（如果没有则可以留空）。

示例：

```js
component MemoryTicketRepository interface TicketRepository
    // the component members //

    Map<Int, String> records

    // the interface methods //

    function Result<Unit, Error> add(Self self, Int id, String issue)
        let ticket = Ticket::new(id, issue)
        self.records.set(id, ticket)
        Ok(Unit::Void)
    end

    function Option<Ticket> get(Self self, Int id)
        self.records.tryGet(id)
    end
end
```

实际上 `组件` 在内部就是一个被标上了额外标记的 `结构体`，上面的代码大致等同于：

```js
struct MemoryTicketRepository
    Map<Int, String> records
end

impl MemoryTicketRepository interface TicketRepository
    // ...
end
```

不过在实现一个接口时，尽量使用 `组件` 而不是 `结构体`，这样目的会更明确。

实例化一个组件时，必须把其所需的所有成员通过默认构造函数传入。使用该组件的示例如下：

```js
let records = Map<Int, String>::new()
let repo = MemoryTicketRepository::new(records)

repo.add(123, "foo")?
repo.add(456, "bar")?

writeLine(repo.get(123) || "")
```

一个接口可以有多个实现（即组件），比如上述的 `TicketRepository` 除了可以有 `MemoryTicketRepository`，还可以有 `SQLiteTicketRepository`, `PostgreSQLTicketRepository` 等等实现。

## 组件环境

一个业务系统往往包含多个接口，且这些接口有依赖关系，为了便于实例化一组接口，XiaoXuan 运行环境自带一个 `组件环境`（`ComponenetContext`），这是一个类似 SpringFramework 的 ApplicationContext IoC 的概念。

假设有一个 `IssueTrack` 系统，系统包含有 `UserRepository`, `StaffRepository`, `TicketRepository`, `SolutionRepository`, `RatingRepository`，其中的依赖关系如下：

```
IssueTrack
  |
  |-- SolutionRepository
  |     |-- TicketRepository
  |     |-- StaffRepository
  |
  |-- RatingRepository
  |     |-- TicketRepository
  |     |-- StaffRepository
  |
  |-- TicketRepository
  |     |-- UserRepository
```

显然，在实例化时需要先实例化 `UserRepository`，`TicketRepository`，然后实例化 `StaffRepository`，再实例化 `SolutionRepository` 和 `RatingRepository`。（前两步的先后顺序可调换）

`组件环境` 是一个组件的容器，我们需要把组件（也就是某个接口的某个实现）依次添加入 `组件环境`。示例：

```js
let context = ComponenetContext::new()

// add UserRepository
let userRepository = MemoryUserRepository::new(
    Map<Int, String, String>::new())
context.add(typeOf(UserRepository), userRepository)

// add TicketRepository
let ticketRepository = MemoryTicketRepository::new(
    Map<Int, String>::new(),
    context.get(typeOf(UserRepository)))
context.add(typeOf(TicketRepository), ticketRepository)
)
```

`组件环境` 提供了 `add` 方法用于添加组件，`get` 方法用于根据接口获取对应的组件。

接下来还要添加 `SolutionRepository` 和 `RatingRepository`，假如它们对应的组件的构造函数仅依赖 `TicketRepository` 和 `StaffRepository` 两个接口的组件，则可以使用宏 `addComponenet` 自动创建组件构造函数所需要的参数，自动调用构造函数，并加入到指定的 `ComponentContext`。示例：

```js
addComponent(context, MemorySolutionRepository)
```

上面一句相当于：

```js
let userRepository = context.get(typeOf(UserRepository))
let ticketRepository = context.get(typeOf(TicketRepository))
let solutionRepository = MemorySolutionRepository::new(userRepository, ticketRepository)
context.add(typeOf(SolutionRepository), solutionRepository)
```
