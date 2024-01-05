use std::num::ParseIntError;

/// Parse a string s to a number
fn main() -> Result<(), ParseIntError> {
    let number_str = "1n0";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
