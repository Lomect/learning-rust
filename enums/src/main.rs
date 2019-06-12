// Option is a enums

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrs {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let x: Option<u8> = Some(5);
    let y: Option<i32> = None;

    if let Some(o) = y {
        println!("o: {}", o);
    } else {
        println!("{:?}", y);
    }

    match home.kind {
        IpAddrKind::V4 => println!("This is V4"),
        IpAddrKind::V6 => println!("This is V6"),
    }

    println!("Hello, world!");
}
