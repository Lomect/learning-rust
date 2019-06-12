
fn main() {
    let mut s = String::from("Hello, world!");
    let l = get_len(&s);
    change(&mut s);
    println!("{} {}", l, s);
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" lomect");
}