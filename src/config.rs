use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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

#[cfg(test)]
mod tests {
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
}
