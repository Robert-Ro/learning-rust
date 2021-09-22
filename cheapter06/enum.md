# Enum
枚举类型，通常也被简称为枚举，它允许我们列举所有可能的值来定义一个类型。

Rust中的枚举类型更类似与F#，OCaml和Haskell这类函数编程语言中的代数数据类型(algebraic data type)。
## 定义枚举
```rs
enum IpAddrKind{ // 标识符
  V4, V6 // 变体
}
```
## 枚举值
```rs
let four IpAddrKind::V4;
```
枚举类型同结构体一样   ，可以使用impl关键字定义枚举的方法。
```rs
impl Message {
  fn call(&self){

  }
}

```
## Option枚举及其在空值处理方面的优势
描述了一种值可能不存在的情形，所以它被广泛地应用在各种地方。

空值代表了因为某种原因而变为无效或缺失的值

Rust中虽然没有空值，但却提供了一个拥有类似概念的枚举，我们可以用它来标识一个值无效或缺失。
这个枚举就是`Option<T>`，它在标准库中被定义为如下所示的样子：
```rs
enum Option<T>{
  Some(T),
  None,
}
```
