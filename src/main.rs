// Crate: this is a binary crate, produces an executable 

// use the input output library
// Rust includes a set of items in every program called prelude
use std::io;
use rand::Rng;// Crate: this is a library crate, usable in binary crates, not executable
use std::cmp::Ordering;// library crate, enum type = {Less, Greater, Equal}

// main is entry point in every program
fn main() {
    // use the println macro with !
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop creates an infinite loop, break it from the match case
    loop {
        println!("Please input your guess.");

        // in Rust, variables are immutable by default
        // let guess = 5;// immutable
        let mut guess = String::new();// mutable

        // io library, stdin is the function
        io::stdin()
            // reference (&) also immutable, use &mut here to make it mutable to guess var
            .read_line(&mut guess)// this returns a Result value
            // Result variants are Ok and Err, expect is an instance method of Result types
            .expect("Failed to read line");

        // shadowing the previously defined guess variable for type conversion
        // .trim eliminates any white space at the beginning and end
        // .parse converts a string to another type
        // u32 = unsigned 32-bit integer, the : allows for type annotation
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,// _ is a catchall value
        };

        println!("You guessed: {guess}");

        // a match is made up of arms, an arm is a pattern to match against and the code that should be run
        // kind of like a switch...case
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }// end of loop
}
