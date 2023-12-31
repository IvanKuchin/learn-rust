use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = std::env::var("CASE_INSENSITIVE").is_ok();


        Ok(Config { query, filename, ignore_case })
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = std::fs::read_to_string(&config.filename)?;

    println!("Search result for: {}", config.query());

    let result = if config.ignore_case() {
        search_case_insensitive(config.query(), &contents)
    } else {
        search(config.query(), &contents)
    };

    println!("{:#?}", result);
    
    Ok(())
}

fn search<'a>(query:&str, content:&'a str) -> Vec<&'a str> {
    content
            .lines()                
            .filter(|line| line.contains(query))
            .collect()
}

fn search_case_insensitive<'a>(query:&str, content:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
            .lines()                
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
    let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}

