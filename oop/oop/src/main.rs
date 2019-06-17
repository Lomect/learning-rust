
/// Trait 对象要求对象安全
///     1. 返回值类型不为 Self
///     2. 方法没有任何泛型类型参数
///

trait Draw {
    fn draw(&self);
}

struct Button {
    high: i32,
    length: i32,
    width: i32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("button: high {}, length {}, width {}", self.high, self.length, self.width);
    }
}

struct Screen<T: Draw> {
    commponts: Vec<Box<T>>,
}

impl<T: Draw> Screen<T> {
    fn run(&self) {
        for s in self.commponts.iter() {
            s.draw();
        }
    }
}

fn main() {
    let sc = Screen{
        commponts: vec![Box::new(Button{
            high: 10,
            length: 10,
            width: 10,
        }),
        Box::new(Button{
            high: 1,
            length: 1,
            width: 1,
        })]
    };
    sc.run();
}
