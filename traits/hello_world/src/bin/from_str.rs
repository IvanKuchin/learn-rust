use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn main() {
    // FILL in the blanks in two ways
    // DON'T change code anywhere else 
    let p = "(3,4)".parse::<Point>();
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );

    let p = Point::from_str("(3,4)");
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );
    
    let p: Result<Point, <Point as FromStr>::Err> = "(33,44)".parse();
    assert_eq!(p.unwrap(), Point{ x: 33, y: 44} );
    
    let p: Result<Point, ParseIntError> = "(33,44)".parse();
    assert_eq!(p.unwrap(), Point{ x: 33, y: 44} );

    assert_eq!("111abc111".trim_matches(|c| c == '1'), "abc");
    println!("Success!");
}
