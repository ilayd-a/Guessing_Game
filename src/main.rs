// by ilayd January 3 20255
use std::io; // input/output library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    // we use let statement to create the variable
    // in rust, variables are immutable by default, to make a variable mutable we add the keyword
    // mut before the variable name
    // guess is a new mutable string variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // if we hadn't imported the io library with use std::io, at the beginning of the program, we
    // could still use the function by writing this function call as std::io::stdin
    println!("You guessed: {}", guess);
    // curly braces are placeholders for variables
}
