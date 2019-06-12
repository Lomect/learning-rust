// use 关键字用来将路径引入作用域
// pub 关键字使项变为公有
// as 关键字用于将项引入作用域时进行重命名
// crate 关键字来开始绝对路径

mod sound {
    pub fn my() {
        println!("My");
    }

    pub mod instrument {
        pub fn data() {
            super::my();
            println!("Pass");
        }
    }
}

mod me {
    pub use crate::sound::instrument;
    pub fn data() {
        println!("Me");
        instrument::data();
    }
}

use sound::{instrument, my, instrument::data};
use sound::*;

fn main() {
    crate::sound::instrument::data();
    sound::instrument::data();
    me::instrument::data();
    me::data();
    println!("Hello, world!");
}

