// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number! (1-100)");

    // The type annotation is not necessary as it is inferred as `u32` due to the comparison with `guess`, [re]defined on line 22, on line 26
    let number/*: u32*/ = rand::thread_rng().gen_range(1, 101);

    // println!("The number is {}.", number);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        // Would have been `std::io::stdin` if I hadn't included `use std::io;`
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if 1 <= num && num <= 100 {
                    num
                } else {
                    println!("Please enter a number between 1 and 100!");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        // println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
