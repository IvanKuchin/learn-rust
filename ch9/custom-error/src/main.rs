use std::{fs::File, io::Read};

#[derive(Debug)]
struct CustomError;

impl From<std::io::Error> for CustomError {
    fn from(_: std::io::Error) -> Self {
        CustomError
    }
}

fn read_username_from_file() -> Result<String, CustomError> {
    let mut f = File::open("hello.txt")?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e.into()),
    // };

    let mut username = String::new();

    // f.read_to_string(&mut username)?;
    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e.into()),
    }

    // Ok(username)
}

fn main() {
    let r = read_username_from_file();
    assert_eq!(r.is_ok(), false);

    match r {
        Ok(s) => println!("success: {}", s),
        Err(e) => println!("error: {:?}", e),
    }

}
