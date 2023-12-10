fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }

    let res : &'static str = "static lifetime string literal";
    res
}

static stc:i32 = 42;

fn main() {
    let string1 = String::from("long string is long");

    let result;
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let r: &'static i32;
    r = &stc;
    // let x = 42;
    // r = &x; // error[E0597]: `x` does not live long enough
}
