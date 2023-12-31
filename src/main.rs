use minigrep::config::Config;
use std::{env, process};

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(values) => values
            .iter()
            .for_each(|occourence| println!("{}", occourence)),
        Err(err) => {
            eprintln!("Application error: {}", err);

            process::exit(1);
        }
    };
}
