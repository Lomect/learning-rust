enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(1,2,3));
    match msg {
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Hsv: ({}, {}, {})",h,s,v);
        },
        Message::ChangeColor(Color::Rgb(r,g,b)) => {
            println!("Rgb: ({},{},{})",r,g,b);
        }
        _ => (),
    };
}
