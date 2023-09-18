use std::{error::Error, fs::File, io::prelude::*};

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        let args_qtt = args.len();

        if args_qtt < 3 {
            return Err("Expected 2 arguments (query, filename)");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename).expect("File could not be not found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("contents found: {}", contents);

    Ok(())
}
