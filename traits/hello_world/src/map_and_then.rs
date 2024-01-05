fn mult(n1_str: &str, n2_str: &str) -> Result<i32, std::num::ParseIntError> {
    let n2 = n2_str
    .parse::<i32>()
    .and_then(|n2| 
        Ok(n2)
    );


    n1_str
        .parse::<i32>()
        .map(|n1| 
            n1 * n2.unwrap()
        )
}

fn mult2(n1_str: &str, n2_str: &str) -> Result<i32, std::num::ParseIntError> {
    n1_str
        .parse::<i32>()
        .map(|n1| 
            (n2_str
            .parse::<i32>()
            .and_then(|n2| 
                Ok(n1*n2)
            )).unwrap()
        )
}

fn main() {
    let result = mult2("21", "2");
    match result {
        Ok(n) => println!("n: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
