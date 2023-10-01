use std::{error::Error, fs::File, io::prelude::*};

pub mod config;

pub fn run(config: config::Config) -> Result<Vec<String>, Box<dyn Error>> {
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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
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
        let config = config::Config::new(iter);

        let _ = run(config.unwrap());
    }
}
