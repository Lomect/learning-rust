extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let num = rand::thread_rng().gen_range(1, 100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Get guess number error!");
        let guess = guess.trim().parse::<u32>().expect("Not a useful number!");
        match guess.cmp(&num) {
            Ordering::Equal => {
                println!("Success, The number is: {}", guess);
                break;
            },
            Ordering::Greater => println!("You Guess Greater!"),
            Ordering::Less => println!("You Guess Less!"),
        }
    }
}
