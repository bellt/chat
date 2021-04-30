use std::env;
use std::process;

use server::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = server::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
