use std::process::{Termination, ExitCode};

struct CustomTermination(i32);

impl Termination for CustomTermination {
    fn report(self) -> ExitCode {
        ExitCode::from(self.0 as u8)
    }
}

fn main() -> CustomTermination {
    println!("Hello, world!");

    CustomTermination(42)
}
