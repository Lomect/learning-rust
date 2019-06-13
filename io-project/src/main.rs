use std::env;
use std::process;
use io_project::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Get Err: {}", e);
        process::exit(1);
    }
}

