use std::{env, fs, process};
use std::error::Error;
use log_parser::search;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    search(contents);

    Ok(())
}

struct Config{
    file_path: String,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {
        let file_path = match args.get(1) {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config{file_path: file_path.to_string()})
    }
}