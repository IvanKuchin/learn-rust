fn main() {
    let x = &false;
    print_type_name_of(x);

    let _y = *x;
    let &y = x;

    let &x = &false;
    print_type_name_of(x);

    let ref x = &false;
    print_type_name_of(x);

    let x = &false;
    print_type_name_of(x);

    let ref x = false;
    print_type_name_of(x);

    let closure = |s: &mut String| 3;

}

fn print_type_name_of<T>(_: T) {
    println!("{}", 1213)
}