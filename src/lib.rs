use std::{
    env::{self},
    error::Error,
    fs::File,
    io::prelude::*,
};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Query argument not provided"),
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Filename argument not provided"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = File::open(config.filename).expect("File could not be found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    Ok(result.into_iter().map(|line| String::from(line)).collect())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod config_tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn return_result_err_when_arg_query_not_provided() {
        let iter = vec!["arg1".to_string()].into_iter();
        let result = Config::new(iter);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Query argument not provided")
    }

    #[test]
    fn return_result_err_when_arg_filename_not_provided() {
        let iter = vec!["arg1".to_string(), "query".to_string()].into_iter();
        let result = Config::new(iter);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Filename argument not provided")
    }

    #[test]
    fn return_ok_along_with_config() {
        let query = "query".to_string();
        let filename = "filename".to_string();
        let iter = ["arg1".to_string(), query.clone(), filename.clone()].into_iter();

        let config = Config::new(iter);

        assert!(config.is_ok());

        let config = config.unwrap();

        assert_eq!(config.query, query);
        assert_eq!(config.filename, filename);
    }

    #[test]
    fn return_ok_given_config() {
        let query = "duct".to_string();
        let test_file = testfile::generate_name();
        let iter = [
            "exe".to_string(),
            query,
            test_file.to_str().unwrap().to_string(),
        ]
        .into_iter();

        let mut file = File::create(&test_file).unwrap();
        file.write_all(
            b"\
Rust:
safe, fast, productive.
duct.
        ",
        );

        let config = Config::new(iter);
        let result = run(config.unwrap());

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec!["safe, fast, productive.".to_string(), "duct.".to_string()]
        );
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
        let iter = [
            "exe".to_string(),
            "query".to_string(),
            "unexisting.txt".to_string(),
        ]
        .into_iter();
        let config = Config::new(iter);

        run(config.unwrap());
    }
}
