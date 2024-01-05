//! check that uninitialized variable is not allowed
 
/// main function must be declared as fn main() -> ()
fn main() {
    let r;
    {
        let x = 5;

        r = &x;
        let ref ref2 = x;
        
        println!("reference to x: {:p}", r);
        println!("reference to x: {:p}", ref2);

    }
    // x does not exist anymore
    // println!("reference to x: {}", r);
}