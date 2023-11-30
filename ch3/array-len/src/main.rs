fn main() {

    let arr = [1, 2, 3, 4, 5];

    let mut idx = String::new();

    println!("Enter the index of the array: ");

    std::io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");

    let idx = idx.trim().parse::<i32>().expect("Please enter a number");

    println!("The value of the array at index {} is {}", idx, arr[idx as usize]);

}
