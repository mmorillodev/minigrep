use std::{env, error::Error, fs::File, io::prelude::*};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        let args_qtt = args.len();

        if args_qtt < 3 {
            return Err("Expected 2 arguments (query, filename)");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename).expect("File could not be found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod config_tests {
    use std::fs::File;

    use super::*;

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
    fn fine_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let result = search(query, contents);

        assert_eq!(&result, &vec!["safe, fast, productive."]);
    }

    #[test]
    fn fine_one_result_when_searching_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let result = search(query, contents);

        assert_eq!(&result, &vec!["safe, fast, productive."]);
    }

    #[test]
    fn fine_two_results_when_searching_case_insensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let result = search_case_insensitive(query, contents);

        assert_eq!(&result, &vec!["Rust:", "Trust me."]);
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
