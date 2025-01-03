// by ilayd January 3 20255
use std::io;// input/output library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    loop {
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
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // rust allows us to shadow the previous value of guess with a new one
        // the parse method will only work on characters that can logically be converted into numbers
        println!("You guessed: {}", guess);
        // curly braces are placeholders for variables
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {println!("You win!"); break;}
            Ordering::Greater => println!("Too big!"),
        }
    }//infinite loop
}
