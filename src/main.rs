use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use rand::Rng;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // allows cursor to stat at the end of the line, manual flush
    write!(stdout, "Press 'Enter' to exit. . .").unwrap();
    stdout.flush().unwrap();

    // read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() { // Result Ok if parse can turn user input to number type, Err if it cannot
            Ok(num) => num,
            Err(_) => continue, // match all Err values
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    pause();

}
