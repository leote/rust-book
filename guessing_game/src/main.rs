use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // secret is random 1-10


    loop{
        println!("Please input your guess: ");

        let mut guess = String::new(); // guess = string variable

        io::stdin()
            .read_line(&mut guess) // call readline method on input
            .expect("Failed to read line"); //  error message

        let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
        


        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // comparing guess to secret number
            Ordering::Less => println!("Too small!"), // if too small
            Ordering::Greater => println!("Too big!"), // if too big
            Ordering::Equal => { // if too equal
                println!("You win!");
                break; // stop when guess correct
            }


        }
    }


}
