use core::fmt;
use std::error;

type CustomResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec(i32);

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double at line {}", self.0)
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> CustomResult<i32> {
    vec
        .first()
        .ok_or_else(|| EmptyVec(20).into())
        .and_then(|v| {
            v.parse::<i32>()
                .map_err(|_| EmptyVec(19).into())
                .map(|i| i*2)
        })
}

fn print(result: CustomResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {n}"),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
