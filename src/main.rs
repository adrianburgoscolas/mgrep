use std::{env, process};
use mgrep::Config;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let config: Config = Config::new(&arg).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

