fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic(expected = "panic!")]
    fn should_pac() {
        panic!("panic!");
    }

    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}