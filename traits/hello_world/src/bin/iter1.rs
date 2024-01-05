fn main() {
    let v = vec![1, 2, 3];
    let _iter = v.iter();

    for i in v.iter() {
        println!("{}", i);
    }

    println!("{:?}", v);
}