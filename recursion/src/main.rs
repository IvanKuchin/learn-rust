
fn cust_loop(n: i32) {
    if n < 1 { return }
    
    println!("{}", n);
    cust_loop(n-1);
    println!("{}", n);
}

fn main() {
    cust_loop(5)
    // cust_loop(7000); // will overflow the stack
}
