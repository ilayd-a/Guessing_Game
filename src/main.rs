use std::io; // input/output library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    // we use let statement to create the variable
    // in rust, variables are immutable by default, to make a variable mutable we add the keyword
    // mut before the variable name
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
