use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //mutable - The :: syntax in the ::new line indicates the new is an associated function of the String type.

    println!("You guessed: {}", guess);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
