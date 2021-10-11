# Rust 通用集合

Rust 标准库包含了一系列非常有用的被称为集合的数据结构。

- 动态数组(`Vector`)可以让你连续地存储任意多个值
- 字符串(`string`)是字符的集合
- 哈希映射(`hash map`)可以让你将值关联到一个特定的键上，他是另外一种数据结构——映射(map)的特殊实现

## 动态数组 Vec<T>

### 创建

```rs
// 方式1
let v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);
// 方式2 简化代码
let v = vec![1, 2, 3,];
```

### 更新

```rs
v.push(T);
```

### 销毁

动态数组一旦离开作用域后会被立即销毁，其中的非引用数据会直接销毁，但是引用数据时需要额外处理

### 读取

- 按索引
- get 方法, 返回值：`Option<&T>`

```rs
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];
println!("The third element is {}", third);
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None=>println!("There is no third element."),
```

#### 边界情况：索引越界

- 按索引获取超出索引的值 => 触发`panic`
- get 方法 => None

#### 所有权问题

> 持有了一个指向动态数组中**首个元素的不可变引用**，但却依然尝试向这个动态数组的结尾处添加元素，该尝试是*不会成功的*
> 动态数组中的元素是**连续存储**的，插入新的元素后也许会没有足够多的空间将所有元素依次相邻地放下，这就**需要分配新的内存空间，并将旧的元素移动到新的空间上**

### 遍历

```rs
let v = vec![1,2,3];
// for
for i in &v{}
for i in &mut v {}
// for
for (index, item) in v.iter().enumerate(){}
```
### [API](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html)
```rs
pub fn pop(&mut self) -> Option<T>
pub fn push(&mut self, value: T)
pub fn capacity(&self) -> usize
pub fn len(&self) -> usize
pub fn clear(&mut self)
```
### 动态数组使用枚举可以存储非同类型的值
为了计算出元素在堆上使用的存储空间，Rust需要在编译时确定动态数组的类型。使用枚举的另一个好处在于它们可以显示地列举出所有可以被放入动态数组的值类型。
## String

字符串：String 与字符串切片&str 两种类型

标准库还提供了诸如：OsString, OsStr, CString 和 CStr 字符串类型。

### 创建

### 更新

### 字符串索引

String 实际上是一个基于`Vec<u8>`的封装类型

UTF8 编码问题

### 字符串切片

### 遍历

### 切割 string

## HashMap

### 创建

hashMap 将数据存储在堆上

### 所有权

对于那些实现了 Copy trait 的类型，如 i32, 它们的值会被简单地复制到哈希映射中。而对于 String 这种持有所有权的值，其值将会转移且所有权会转移给哈希映射。

### 访问

### 遍历

### 更新

#### K 已经存在，对应一个 V

- 替换现有的 V
- 保留现有的 V，忽略新的 V
- 合并现有的 V 和新的 V

#### K 不存在

- 添加一对 K,V
