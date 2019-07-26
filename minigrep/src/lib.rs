use std::error::Error;
use std::fs;
// The point of making this a separate file is obviously to factor out the logic from
// the main loop

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read in the file specified by the cli args
    let contents = fs::read_to_string(config.filename)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

// We're implementing a config ?class here with an constructor (new). We can subsequently
// call this like Config::new...
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arugments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// This is a simple struct for returning config as passed to the cli
pub struct Config {
    query: String,
    filename: String,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Declare a new mutable vector
    let mut results = Vec::new();

    for line in contents.lines() { // Returns an iterator
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Apparently this is how you write tests in rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
