struct User {
    name: String,
    age: u32,
    email: String,
    active: bool
}

struct Rec {
    with: u32,
    leng: u32,
}

impl Rec {
    // 不以self作为第一个参数的函数称为: 关联函数
    fn new(leng: u32) -> Self {
        Rec{
            with: leng,
            leng
        }
    }
    fn area(&self) -> u32 {
        self.with * self.leng
    }
}

fn main() {
    let user = User{
        name: String::from("Lome"),
        age: 27,
        email: String::from("google@qq.com"),
        active: false
    };

    let user2 = User{
        name: String::from("Lory"),
        ..user
    };

    let rec = Rec::new(20);
    println!("{}", rec.area());
}

fn build_user(name: String, email: String) -> User {
    User{
        name,
        email,
        age: 20,
        active: false
    }
}