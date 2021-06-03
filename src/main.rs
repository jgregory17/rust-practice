use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!"); // ! means running macro instead of executing function

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // vars are immutable by default

        io::stdin()
            .read_line(&mut guess) // & symbol means passing var by ref
            .expect("Failed to read line"); // displays message when read_line returns Err variant of Result

        // Ok and Err are variants of Result enum
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match is like 'switch'
        // cmp returns variant of Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
