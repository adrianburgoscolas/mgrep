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

pub struct Config {
    pub path: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query parameter..."),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No path parameter..."),
        };
        let case_sensitive: bool = env::var("IGNORE_CASE").is_ok();
        Ok(
            Config {
                query,
                path,
                case_sensitive,
            }
        )
    }
}

pub fn search<'a>(query: &'a str, text: &'a str) -> Vec<&'a str>{
    text.lines().filter(|line| line.contains(query)).collect()
}

pub fn case_insensitive_search<'a>(query: &'a str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
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
