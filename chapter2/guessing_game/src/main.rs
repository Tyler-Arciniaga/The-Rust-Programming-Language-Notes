use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    let target = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
