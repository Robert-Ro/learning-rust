# Struct

结构，或者收结构体，是一种自定义数据类型，允许我们命名多个相关的值并将他们组成一个邮寄的结合体。

> Rust 中类的模拟实现

## 定义并实例化结构体

需要为每个数据赋予名字以便清楚地表明它们的意义。

```rs
// 定义
struct User{
  <field_name>: <type>
}
// 创建
let user = User{
  <field_name>: <field_value>
}
// 访问
println!("user.<field_name>: {}", user.<field_name>);
```

### 备注

- **一旦实例可变，那么实例中的所有字段都将是可变的**。
- 使用函数构建，若参数名与字段名相同，可简化书写
- 创建实例时，可以使用`..<struct>`的方式注入已有实例的属性

### 元组结构体
```rs
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
let black = Color(0,0,0);
let origin = Point(0,0,0);
// black与origin属于不同的类型
```
### 空结构体
```rs
struct Empty{}
```
### 结构体的所有权
- 可以在结构体中存储指向其他数据的引用，需要用到生命周期功能
## 方法
### 定义
```rs
impl struct_name {
  fn method_name(&self||&mut self) -> <return_type> {
    // 访问self.x
  } // 使用：struct_name.method_name()
  // 关联函数
  fn method_name(args: type) -> <return_type> {} // struct_name::method_name()
}
```
### Rust自动引用和解引用

> 当你使用`object.something()`调用方法时，`Rust`会自动为调用者`object`添加`&`、`&mut`或`*`，以使其能够符合方法的签名
## 总结

结构体可以让我们基于特定领域创建有意义的自定义类型。通过使用结构体，你可以将相关联的数据组合起来，并为每条数据赋予名字，从而使代码变得更加清晰。方法可以让我们为结构体实例指定行为，而关联函数则可以将哪些不需要实例的特定功能放置到结构体的命名空间中。

结构体不是创建自定义类型的唯一方法，还有枚举。
