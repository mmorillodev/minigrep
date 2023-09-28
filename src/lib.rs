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
    let mut file = File::open(config.filename).expect("File could not be found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("contents found: {}", contents);

    Ok(())
}

#[cfg(test)]
mod config_tests {
    use std::fs::File;

    use super::run;

    use super::Config;

    #[test]
    fn return_result_err_when_args_len_is_lower_than_three() {
        let result = Config::new(&[String::from("arg1"), String::from("arg2")]);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Expected 2 arguments (query, filename)"
        )
    }

    #[test]
    fn return_ok_along_with_config() {
        let query = String::from("query");
        let filename = String::from("filename");

        let config = Config::new(&[String::from("exe"), query.clone(), filename.clone()]);

        assert!(config.is_ok());

        let config = config.unwrap();

        assert_eq!(config.query, query);
        assert_eq!(config.filename, filename);
    }

    #[test]
    fn return_ok_given_config() {
        let test_file = testfile::generate_name();
        let _ = File::create(&test_file);

        println!("Creating file with name: {}", test_file.to_str().unwrap());

        let config = Config::new(&[
            String::from("exe"),
            String::from("query"),
            String::from(test_file.to_str().unwrap()),
        ]);

        let result = run(config.unwrap());

        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "File could not be found!")]
    fn panic_when_file_not_found() {
        let config = Config::new(&[
            String::from("exe"),
            String::from("query"),
            String::from("unexisting.txt"),
        ]);

        let _ = run(config.unwrap());
    }
}
