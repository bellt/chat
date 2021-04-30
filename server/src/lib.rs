use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub name: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("Server name required"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Need filename"),
        };

        Ok(Config {name, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let chat_contents = fs::read_to_string(config.filename)?;

    println!("{}", chat_contents);

    Ok(())
}