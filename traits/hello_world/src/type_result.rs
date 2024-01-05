#[derive(Debug, PartialEq)]
struct Fail(String);

struct Ok<T>(T);

enum Result<T> {
    Ok(T),
    Fail(String),
} 

struct RetStr(&'static str);

fn check_status(code: bool) -> Fail {
    match code {
        true => Fail("Success".to_string()),
        false => Fail("Failure".to_string()),
        // _ => Fail("Unknown".to_string()),
    }
}

fn test_ret_str() -> RetStr {
    RetStr("Hello")
}

fn main() {
    let Fail(message) = check_status(false);
    println!("Message is: {}", message);

    let f1 = Fail("Failure".to_string());
    let f2 = Fail("Failure".to_string());

    assert_eq!(f1, f2);

    let at: Result<i32> = Result::Fail("that sucks".to_string());

    match at {
        Result::Ok(x) => println!("ok - {}", x),
        Result::Fail(str) => println!("fail - {}", str),
    }

    println!("RetStr: {}", test_ret_str().0);

    println!("Success");
}