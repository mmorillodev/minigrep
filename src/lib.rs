use colored::*;
use std::{error::Error, fmt, fs::File, io::prelude::*};

pub mod config;

#[derive(Debug, PartialEq)]
pub struct GrepResult {
    pub filename: String,
    pub occourences: Vec<GrepOccourence>,
}

#[derive(Debug, PartialEq)]
pub struct GrepOccourence {
    pub line_number: u32,
    pub content: String,
}

impl GrepResult {
    fn new(filename: String, occourences: Vec<GrepOccourence>) -> Self {
        Self {
            filename,
            occourences,
        }
    }
}

impl GrepOccourence {
    fn new(line_number: u32, content: String) -> Self {
        Self {
            line_number,
            content,
        }
    }
}

impl fmt::Display for GrepOccourence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.line_number.to_string().green(),
            self.content
        )
    }
}

impl fmt::Display for GrepResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.filename.cyan(),
            self.occourences
                .iter()
                .fold(String::new(), |acc, occourence| acc
                    + "\n"
                    + &occourence.to_string())
        )
    }
}

pub fn run(config: config::Config) -> Result<GrepResult, Box<dyn Error>> {
    let mut file = File::open(&config.filename).expect("File could not be found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    let grep_result = GrepResult::new(config.filename, result);

    Ok(grep_result)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<GrepOccourence> {
    let mut occourences = vec![];

    for (line_number, content) in contents.lines().enumerate() {
        if content.contains(&query) {
            occourences.push(GrepOccourence::new(
                (line_number + 1) as u32,
                content.to_string(),
            ));
        }
    }

    occourences
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<GrepOccourence> {
    let mut occourences = vec![];
    let query = query.to_lowercase();

    for (line_number, content) in contents.lines().enumerate() {
        if content.to_lowercase().contains(&query) {
            occourences.push(GrepOccourence::new(
                (line_number + 1) as u32,
                content.to_string(),
            ));
        }
    }

    occourences
}

#[cfg(test)]
mod lib_tests {
    use std::fs::File;

    use super::*;

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
        let _ = file.write_all(
            b"\
Rust:
safe, fast, productive.
duct.
        ",
        );

        let config = config::Config::new(iter);
        let result = run(config.unwrap());
        let expected_result = GrepResult::new(
            test_file.to_str().unwrap().to_string(),
            vec![
                GrepOccourence::new(2, "safe, fast, productive.".to_string()),
                GrepOccourence::new(3, "duct.".to_string()),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn fine_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected_result = vec![GrepOccourence::new(
            2,
            "safe, fast, productive.".to_string(),
        )];

        let result = search(query, contents);

        assert_eq!(&result, &expected_result);
    }

    #[test]
    fn fine_one_result_when_searching_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let expected_result = vec![GrepOccourence::new(
            2,
            "safe, fast, productive.".to_string(),
        )];

        let result = search(query, contents);

        assert_eq!(&result, &expected_result);
    }

    #[test]
    fn fine_two_results_when_searching_case_insensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let expected_result = vec![
            GrepOccourence::new(1, "Rust:".to_string()),
            GrepOccourence::new(4, "Trust me.".to_string()),
        ];

        let result = search_case_insensitive(query, contents);

        assert_eq!(&result, &expected_result);
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
        let config = config::Config::new(iter);

        let _ = run(config.unwrap());
    }
}
