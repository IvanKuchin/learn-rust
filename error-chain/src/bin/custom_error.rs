// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
        //     Io(::std::io::Error) #[doc = "Error during IO"];
        }
    
        errors {
            // Define additional errors. The syntax here is the same as
            // `quick_error!`, but the `from()` and `cause()` syntax is
            // not supported.
            AnotherError {
                description("description of another error")
                display("display of another error")
            }
            RunError {
                description("description - unable to finish run()")
                display("display - unable to finish run()")
            }
        }
    }
}

use error_chain::ChainedError;
use errors::*;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter() {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        // if let Some(backtrace) = e.backtrace() {
        //     println!("backtrace:\n{:?}", backtrace);
        // }

        println!("-----------------------");
        println!("{}", e.display_chain());

        std::process::exit(1);
    }
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    // This operation will fail
    try_to_open_file()
        .chain_err(|| ErrorKind::RunError)?;

    throw_custom_error()
        .chain_err(|| "unable to finish run() due to custom error")?;

    Ok(())
}

fn try_to_open_file() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("contacts")
        .chain_err(|| "unable to open contacts file")?;

    Ok(())
}

fn throw_custom_error() -> Result<()> {
    Err(ErrorKind::AnotherError.into())
}