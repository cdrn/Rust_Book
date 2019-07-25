use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read in the file specified by the cli args
    let contents = fs::read_to_string(config.filename)?;
    
    println!("Our text is: {}", contents);

    Ok(())
}

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

