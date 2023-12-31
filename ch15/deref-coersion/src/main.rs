use std::{str::FromStr, ops::Deref};

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

fn custom_print(s: &Vec<String>) {
    println!("{:?}", s);
}

fn main() {
    let x = vec![String::from_str("hello").unwrap(), String::from_str("world").unwrap()];
    println!("&Vec<String>");
    custom_print(&x);
    
    let _y = MyBox::new(x);
    println!("&MyBox<Vec<String>>");
    custom_print(&_y);
}
