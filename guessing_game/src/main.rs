use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Loop loops infinitely until break is called
    loop {
        println!("Please input a guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        // Rust allows us to shadow variables by redeclaring with let
        // This allows us to not have to redeclare guess but still parse it to
        // an integer
        let guess: u32 = match guess.trim().parse() {  // Trim removes whitespace, parse parses to number
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed {}", guess)
    }
}
