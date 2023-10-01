use colored::*;
use std::{
    error::Error,
    fmt,
    fs::{read_dir, File},
    io::prelude::*,
    path::Path,
};

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
            "{}:{}",
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

pub fn run(config: config::Config) -> Result<Vec<GrepResult>, Box<dyn Error>> {
    let path = Path::new(&config.filename);
    let mut result: Vec<GrepResult> = Vec::new();

    scan_recursive(&path, &config, &mut result)?;

    Ok(result)
}

fn scan_recursive(
    path: &Path,
    config: &config::Config,
    occourences: &mut Vec<GrepResult>,
) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        let dir_iter = read_dir(path)?;

        for item in dir_iter {
            let path = item?.path();

            let _ = scan_recursive(path.as_path(), config, occourences);
        }

        return Ok(());
    }

    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let file_occourences = if config.case_sensitive {
        search(&config.query, &buffer)
    } else {
        search_case_insensitive(&config.query, &buffer)
    };

    if let Some(file_occourences) = file_occourences {
        occourences.push(GrepResult::new(
            path.to_str().unwrap().to_string(),
            file_occourences,
        ));
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Option<Vec<GrepOccourence>> {
    let mut occourences = vec![];

    for (line_number, content) in contents.lines().enumerate() {
        if content.contains(&query) {
            occourences.push(GrepOccourence::new(
                (line_number + 1) as u32,
                content.to_string(),
            ));
        }
    }

    if occourences.is_empty() {
        None
    } else {
        Some(occourences)
    }
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Option<Vec<GrepOccourence>> {
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

    if occourences.is_empty() {
        None
    } else {
        Some(occourences)
    }
}

#[cfg(test)]
mod search_sensitive_test {

    use super::*;

    #[test]
    fn fine_one_result() {
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

        assert_eq!(&result, &Some(expected_result));
    }

    #[test]
    fn return_none_when_no_occourence_found() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let result = search(query, contents);

        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod search_insensitive_test {

    use super::*;

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

        assert_eq!(&result, &Some(expected_result));
    }

    #[test]
    fn return_none_when_no_occourence_found() {
        let query = "roses";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let result = search_case_insensitive(query, contents);

        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod scan_tests {
    // TODO: Create tests for directory scan
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
        assert_eq!(result.unwrap(), vec![expected_result]);
    }

    #[test]
    fn return_error_when_file_does_not_exist() {
        let iter = [
            "exe".to_string(),
            "query".to_string(),
            "unexisting.txt".to_string(),
        ]
        .into_iter();
        let config = config::Config::new(iter);

        let result = run(config.unwrap());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No such file or directory (os error 2)"
        )
    }
}
