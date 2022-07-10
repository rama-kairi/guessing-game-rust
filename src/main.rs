use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guessing_game = GuessingGame::new();
    guessing_game.play();
}

// Create a struct "GuessingGame"
struct GuessingGame {
    secret_number: u32,
    counter: u32,
}

impl GuessingGame {
    // Create a new instance of the struct "GuessingGame"
    fn new() -> GuessingGame {
        GuessingGame {
            secret_number: rand::thread_rng().gen_range(1..=100),
            counter: 0,
        }
    }

    // Create a function "guess" that takes a single argument "guess"
    fn play(&mut self) {
        loop {
            // Increment the counter by 1
            self.counter += 1;

            // Defining mutable variables "guess"
            let mut guess = String::new();

            if self.counter == 1 {
                println!("\nGuess a number between 1 and 100!");
            }

            // Taking input from the user with "io::stdin"
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // Convert the string "guess" to an integer with "guess.trim().parse()"
            let guess: u32 = guess.trim().parse().expect("Please type a number!");

            // if the Guess is not between 1-100 then print "Please enter a number between 1 and 100"
            if guess < 1 || guess > 100 {
                println!("Please enter a number between 1 and 100");
            } else {
                // Compare the guess to the secret number
                match guess.cmp(&self.secret_number) {
                    Ordering::Less => {
                        println!(
                            "Too small !!! Guess Again! --- Guess Count {}",
                            self.counter
                        )
                    }
                    Ordering::Greater => {
                        println!("Too big !!! Guess Again! --- Guess Count {}", self.counter)
                    }
                    Ordering::Equal => {
                        println!(
                            "Wow 👍👍👍👍👍!!! You guessed it right in {} attempts!",
                            self.counter
                        );
                        break;
                    }
                }
            }

            // if the Guess Count is more than 6 then print "🍌 You loose the game 😭😭😭😭😭!!!"
            if self.counter > 6 {
                println!("🍌 You loose the game 😭😭😭😭😭!!!");
                break;
            }
        }
    }
}
