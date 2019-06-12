// 常量
const  MAX_POINTS : u32 = 1000_000;

fn main() {
    // 不可变变量
    let a = 5;
    // 可变变量
    let mut x = 6;
    x = 7;
    // 隐藏
    let x = x +1;
    println!("a: {}, x: {}", a, x);
}
