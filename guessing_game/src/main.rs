use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // let secret_number: u8 = random(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); //mutable - The :: syntax in the ::new line indicates the new is an associated function of the String type.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // convert guess from string type to integer type
    let guess: u32 = guess.trim().parse().expect("Please type a  number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win 😀"),
    }
}
