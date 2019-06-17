/// Rust 中最常见的指针是第四章介绍的引用, 引用是一类只借用数据的指针
/// 智能指针的显著特性在于其实现了 Deref 和 Drop trait.
/// Deref trait 允许智能指针结构体实例表现的像引用
/// Drop trait 允许我们自定义当智能指针离开作用域时运行的代码
///
/// Box<T>，用于在堆上分配值
/// Rc<T>，一个引用计数类型，其数据可以有多个所有者
/// Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型
///

/// Deref
///

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

//
//
//impl List {
//    fn tail(&self) -> Option<&RefCell>
//}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

use std::ops::Deref;

///为了启用 * 运算符的解引用功能，需要实现 Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(he: &str) {
    println!("{}", he);
}
/// 当 T: Deref<Target=U> 时从 &T 到 &U
/// 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U
/// 当 T: Deref<Target=U> 时从 &mut T 到 &U


/// Drop
///

use std::ops::Drop;

struct Customer {
    data: String,
}

impl Drop for Customer {
    fn drop(&mut self) {
        println!("Drop Customer!");
    }
}

use std::rc::Rc;

enum Enum {
    Conss(i32, Rc<Enum>),
    NIlS,
}

use Enum::{Conss, NIlS};

/// 对于引用和 Box<T>，借用规则的不可变性作用于编译时。对于 RefCell<T>，这些不可变性作用于 运行时。
/// 类似于 Rc<T>，RefCell<T> 只能用于单线程场景
///
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
    let me = MyBox::new(6);
    println!("{}", *me);
    let he = MyBox::new(String::from("Rust"));
    /// 这里使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。
    /// 1.在 MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
    /// 2.标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。Rust 再次调用 deref 将 &String 变为 &str
    hello(&he);
    println!("Hello, world!");

    let cu = Customer{ data: String::from("Lome")};
    let lory = Customer{ data: String::from("lory")};
    drop(cu);
    println!("Finish!");

    let en = Rc::new(Conss(3, Rc::new(Conss(4, Rc::new(Conss(5, Rc::new(NIlS)))))));
    let a = Rc::new(Conss(8, Rc::clone(&en)));
    let b = Rc::new(Conss(7, Rc::clone(&en)));
}
