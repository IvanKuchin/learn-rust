use std::{str::FromStr, ops::Deref, fmt};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl fmt::Display for MyBox<Vec<String>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let s = self.to_string();
        let len = s.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", s);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for MyBox<Vec<String>> {}

fn deref_coersion(s: &Vec<String>) -> String {
    format!("{:?}", s)
}

fn main() {
    let x = vec![String::from_str("hello").unwrap(), String::from_str("world").unwrap()];
    println!("&Vec<String>: {:?}", x);
    
    let _y = MyBox::new(x.clone());
    println!("&MyBox<Vec<String>>: {:?}", _y);
    println!("&MyBox<Vec<String>> deref coersion: {}", deref_coersion(&_y));

    _y.outline_print();
}
