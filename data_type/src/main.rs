
fn main() {
    // 浮点型
    let x = 2.0;
    let y: f32 = 6.0;
    // 运算
    let a = 2.0 +3.0;
    let b = 4.0 -2.0;
    let c = 3.0*3.0;
    let d = 6.0/3.0;
    let e = 9.0/2.0;
    let f = true;
    let g = false;
    let h = 'ℤ';
    let tup: (i32, f64, char) = (12, 6.0, 'c');
    let (x, y, z) = tup;
    let i = tup.0;
    let h = [1,2,3,4,5];
    let j = tup.1;
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);
}
