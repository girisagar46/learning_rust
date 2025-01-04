use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guess.");

        // Eventhough `guess` is already defined, we can shadow re-use the variable name
        // `guess.trim().parse()` returns `Result` type, so we can use `match` to handle the result
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // In case of error input (e.g. non-integer), ignore and continue the loop
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
}
