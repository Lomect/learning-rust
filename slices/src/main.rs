
// slices 是另一个没有所有权的数据类型.

fn main() {
    let hello = String::from("Hello, world!");
    let word = first_word(&hello);
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}