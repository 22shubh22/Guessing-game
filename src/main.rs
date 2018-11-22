extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut moves : u32 = 0;
    println!("Guess the number!");

    let secret_number : u32 = rand::thread_rng().gen_range(1, 101);

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();
        
        //take guess input, but as a string and newline character at end.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        moves = moves + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Total steps: {}",moves);
                break;
            },
        }

    }
}
