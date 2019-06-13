use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let result = search(&config.query, &content);
    println!("With:");
    for s in result {
        println!("{}", s);
    }
    Ok(())
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(f) => f,
            None => return Err("Not Get Query!"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Not Get Filename!"),
        };
        Ok(Config{ query, filename})
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|s| s.to_lowercase().contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let query = "lome";
        let content = "\
Lome is a good man,
he is gyj and like rustlang
lome is a good name
and rust is trust";
    assert_eq!(vec!["Lome is a good man,", "lome is a good name" ], search(query, content));
    }
}