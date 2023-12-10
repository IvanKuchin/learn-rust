fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest =item;
        }
    }

    largest
}

fn main() {
    let list = vec![34, 50, 25, 100, 65];

    println!("Vector capacity is {}", list.capacity());
    // list[3] = 200;

    let mut largest = &list[0];

    println!("The first number is {}", largest);

    for item in &list {
        if item > largest {
            largest =item;
            // println!("{:p}", item);
        }
    }

    println!("The largest number is {}", list[3]);
    println!("The pointer to largest number is {:p}", largest);
    println!("The pointer to list[3] is {:p}", &list[3]);
    
    println!("The largest number in a slice is {:p}", &(find_largest(vec![34, 50, 25, 100, 65].as_slice())));
    println!("The largest number in a slice is {:p}", &find_largest(vec![34, 50, 25, 100, 65].as_slice()));

}
