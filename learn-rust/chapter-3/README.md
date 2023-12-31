一、Rust 变量和可变性：

1、Rust 中变量默认是不可改变的（immutable），也就是说当你定义一个变量，将于绑定的参数后，就不能作出改变它的动作如下：

```rust
fn main() {
    let s = "hello";
    println!("{}", s);
    s = "world";
    println!("{}", s1);
}
```

运行时候，报错指出：不能将两次赋值给不可变变量 ‘ s’

2、可变性
如果想要对 s 进行改变，报错也指出了，需要用到 mut

```rust
fn main() {
    let mut s1 = "hello";
    s1 = "world";
    println!("{}", s1);
}
```

二、常量
Rust 的常量，有点类似于不可变变量，绑定到一个名称的不允许改变的值，可以仔细研究它与不可变变量有什么区别
常量定义：

```rust
fn main() {
    const TOKEN: &str = "sfdgesdf43ytrg967uhf";
    println!("{}", TOKEN);
}
```

首先，不允许对常量使用 mut。常量不光默认不可变，它总是不可变。
声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型。

三、类型

四、函数

在 Rust 语言中，函数的返回值是函数体最后一个表达式的值。所以，对比下面两个函数写法区别，以及可以写出一个错误函数

```rust
fn main() {
    let s = sum(12, 12);
    println!("{}", s);
    println!("{}", sum1(1, 2));
}

fn sum(i1: i32, i2: i32) -> i32 {
    let s = i1 * i2;
    s
}

fn sum1(i1: i32, i2: i32) -> i32 {
    i1 * i2
}
```
