// 堆栈
// 堆上可以分配没有固定大小内存, 栈上必须分配有固定大小的内存
// 这就使得在堆上分配内存的变量里存着的是,其在堆上的指针.

fn main() {
    let x = 10;
    let y = x;
    let s1 = String::from("My name is lome");
    let s2 = s1.clone(); // 如果去掉clone方法就会发生错误,因为所有权系统将s1的所有权转移给s2, s1被回收.
    println!("{} {} {} {}", x, y, s1, s2);
    // x能用是因为存储在栈上的类型都实现了Copy特性.例如: u32 , bool , f32, char, (i32, i32)
    // 所有权函数
    let s = String::from("Hello World");
    prints1(&s); // here s is borrowed the value not moved.
    prints(s); // here s is been moved
//    println!("{}", s);
}

fn prints(s : String)  {
    println!("{}", s);
}

fn prints1(s: &String) {
    println!("{}", s);
}
