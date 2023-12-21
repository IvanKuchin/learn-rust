fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let numbers  = "0123456789ABCDEFGHIJKLMNOP";

    for i in 0..alphabet.len() {
        let mirror = alphabet[..i].chars().zip(numbers[i..].chars().rev()).collect::<Vec<_>>();

        println!("{:?}", mirror);
    }
}