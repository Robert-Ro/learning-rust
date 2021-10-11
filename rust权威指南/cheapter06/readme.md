# 枚举与模式匹配

## Enum

枚举类型，通常也被简称为枚举，它允许我们列举所有可能的值来定义一个类型。

Rust 中的枚举类型更类似与`F#`，`OCaml`和`Haskell`这类函数编程语言中的代数数据类型(`algebraic data type`)。

### 定义枚举

```rs
enum IpAddrKind{ // 标识符
  V4, V6 // 变体
}
```

### 枚举值

```rs
let four = IpAddrKind::V4;
```

枚举类型同结构体一样，可以使用`impl`关键字定义枚举的方法。

```rs
impl Message {
  fn call(&self){
  }
}
```

### 枚举变体数据

枚举允许直接将其**关联的数据**嵌入枚举变体中

```rs
enum IpAddr{
  V4(String), V6(String),
}
```

### 定义枚举的方法

```rs
impl Message{
  fn call(&self){ //方法体
  }
  fn call2() { //方法体
  }
}
```

数据类型可以是：任意数据(基础类型+结构体+元组+枚举)

### Option 枚举及其在空值处理方面的优势

`Option`描述了一种值可能不存在的情形，所以它被广泛地应用在各种地方。

**空值代表了因为某种原因而变为无效或缺失的值**

Rust 中虽然没有空值，但却提供了一个拥有类似概念的枚举，我们可以用它来标识一个值无效或缺失。
这个枚举就是`Option<T>`，它在标准库中被定义为如下所示的样子：

```rs
enum Option<T>{
  Some(T), // 值存在
  None, // 值不存在
}
```

在不加`Option::`前缀的情况下直接使用`Some`或`None`

在编写代码的过程中，**不必再去考虑一个值是否为空**可以极大地增强我们对自己代码的信心。

_为了持有一个可能为空的值_，我们总是需要将它显式地放入对应类型的`Option<T>`值中。当我们随后使用这个值的时候，也必须显式地处理它可能为空的情况。

无论在什么地方，**只要一个值的类型不是`Option<T>`的，我们就可以安全地假设这个值不是非空的**。

### 获取 Some(T)中的值

**match 表达式**就是这么一个可以用来处理枚举的控制流结构：它允许我们：

- 基于枚举拥有的变体来决定运行的代码分支
- 并允许代码通过匹配值来获取变体内的数据

## 控制流运算符 match

它允许将一个值与一系列的模式相比较，并根据匹配的模式执行相应的代码。

> match 表达式想象成一台硬币分类机：硬币滑入有着不同大小孔洞的轨道，并且掉入第一个符合大小的孔洞。同样，值也会依次通过 match 中的模式，并且在遇到第一个“符合”的模式时进入相关联的代码块，并在执行过程中被代码所使用

### 绑定值的模式

- 匹配的分支可以绑定到被匹配对象的部分值
  - 因此，可以从 enum 变体中提取值

```rs
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

### 匹配 Option<T>

```rs
// 举例
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

```rs
match x {
}
```

### 必须穷举所有的可能性

### \_通配符

表示其余的所有可能性

### if let

- 处理只关心一种匹配而忽略其他匹配的情况
- 更少的代码，更少的缩进，更少的模板代码。
- 放弃了穷举的可能
- 可以把`if let`看作时`match`的语法糖
- 搭配`else`使用
