use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct PanCakes2 {}

fn main() {
    PanCakes2::hello_macro();
}