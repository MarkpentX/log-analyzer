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

    match config.level_filter {
        Some(level_filter) => {
            search(contents, Some(level_filter.filter_by.as_str().to_string()));
        }
        None => {
            search(contents, None);
        }
    }

    Ok(())
}

enum LevelFilterTypes{
    Info,
    Warn,
    Error,
}

impl LevelFilterTypes {
    fn as_str(&self) -> &'static str {
        match self {
            LevelFilterTypes::Info => "INFO",
            LevelFilterTypes::Warn => "WARN",
            LevelFilterTypes::Error => "ERROR",
        }
    }
}

struct LevelFilter {
    filter_by: LevelFilterTypes,
}

struct Config{
    file_path: String,
    level_filter: Option<LevelFilter>,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {

        let file_path = match args.get(args.len() - 1) {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        if let Some(pos) = args.iter().position(|x| x == "--level") {
            if let Some(value) = args.get(pos + 1) {

                let filter = match value.as_str() {
                    "INFO" => LevelFilterTypes::Info,
                    "WARN" => LevelFilterTypes::Warn,
                    "ERROR" => LevelFilterTypes::Error,
                    _ => return Err("Unknown log level"),
                };

                return Ok(Config {
                    file_path: file_path.to_string(),
                    level_filter: Some(LevelFilter {
                        filter_by: filter,
                    }),
                });
            }
        }

        Ok(Config{file_path: file_path.to_string(), level_filter: None })
    }
}