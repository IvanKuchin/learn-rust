use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = std::fs::read_to_string(&config.filename)?;
    println!("With text:\n{}", contents);

    Ok(())
}

