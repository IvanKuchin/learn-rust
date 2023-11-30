use std::{io, cmp::Ordering};
use rand::{thread_rng, Rng};
// use std::cmp::Ordering;

#[derive(Debug)]
enum CustomError {
    Io(io::Error),
    ParseInt(std::num::ParseIntError),
}

fn main() -> Result<(), CustomError> {
    println!("Welcome to the game arena");

    let secret_number = thread_rng().gen_range(1..101);

    println!("The secret number is: {secret_number}");

    loop {

        println!("Please guess the number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess).unwrap();

        println!("You guessed: {guess}");

        let guess = match guess.trim().parse::<u32>() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                },
            };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You win");
                break;
            },
            _ => println!("You lose"),
        }

    }


    Ok(())
}
