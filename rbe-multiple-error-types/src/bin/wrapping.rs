use std::{num::ParseIntError, fmt::{Formatter, Error}};
use std::io::Write;


#[derive(Debug)]
enum DoubleErr {
    Empty,
    Parse(ParseIntError),
}

impl std::error::Error for DoubleErr {}

impl std::fmt::Display for DoubleErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            DoubleErr::Empty => {
                let f = write!(f, "first element should not be empty");
                f
            },
            DoubleErr::Parse(e) => write!(f, "can't parse to int {}", e),
        }
    }
}

impl From<ParseIntError> for DoubleErr {
    fn from(err: ParseIntError) -> DoubleErr {
        DoubleErr::Parse(err)
    }
}

fn first_doubled(vectors: Vec<&str>) -> Result<i32, DoubleErr> {
    let first = vectors
        .first()
        // .ok_or(DoubleErr::Empty)?;
        .ok_or_else(|| {
            log::error!("vectros.first() failed"); 
            DoubleErr::Empty
        })?;

    let parsed = first
        .parse::<i32>()?;
        // .map_err(|e| DoubleErr::Parse(e))?;

    Ok(parsed * 2)
}

fn print(r : Result<i32, DoubleErr>) {
    match r {
        Ok(mult) => println!("first doubled is {mult}"),
        Err(e) => println!("ERROR: {}", e),
    }
}

fn init_logger() {
    let mut builder = env_logger::Builder::new();

    builder.format(|buf, record| {
            writeln!(buf, "{}:{} {} [{}] - {}", 
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(), 
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Trace)
        // .parse_default_env()
        .init();
}

fn main() {
    init_logger();

    let numbers = vec!["42", "77"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["nuts", "10", "100"];

    print(first_doubled(numbers));
    print(first_doubled(empty));
    print(first_doubled(strings));
}
