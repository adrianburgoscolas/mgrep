use std::{env, process};
use mgrep::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

