use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.path)?;
    let lines: Vec<&str>;
    if config.case_sensitive {
        lines = case_insensitive_search(&config.query, &content)
    } else {
        lines = search(&config.query, &content)
    }
    for line in lines {
        println!("{line}");
    }
    Ok(())
}

pub struct Config<'a> {
    pub path: &'a str,
    pub query: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(arg: &Vec<String>) -> Result<Config, &str> {
        if arg.len() < 3 {
            return Err("Not enough arguments...");
        }
        let case_sensitive: bool = env::var("IGNORE_CASE").is_ok();
        Ok(
            Config {
                query: &arg[1],
                path: &arg[2],
                case_sensitive,
            }
        )
    }
}

pub fn search<'a>(query: &'a str, text: &'a str) -> Vec<&'a str>{
    let mut lines: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.contains(query) {
            lines.push(line);
        }
    }
    lines
}

pub fn case_insensitive_search<'a>(query: &'a str, text: &'a str) -> Vec<&'a str> {
    let mut lines: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            lines.push(line);
        }
    }
    lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "ola";
        let text: &str = "\
Bienvenidos
hola, como estan las cosas
por alla todo bien.
Ola marina gigante.";
        assert_eq!(vec!["hola, como estan las cosas"], search(query, text));
    }
    #[test]
    fn case_insensitive() {
        let query: &str = "Ola";
        let text: &str = "\
Bienvenidos
hola, como estan las cosas
por alla todo bien.
Ola marina gigante.";
        assert_eq!(vec!["hola, como estan las cosas","Ola marina gigante."], case_insensitive_search(query, text));
    }
}
