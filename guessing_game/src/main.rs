use std::cmp::Ordering;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use slowprint::slow_print;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    while attempts < 5 {
        println!("Guess the number!");


        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Reassign guess input to unsigned 32 bit integer
        // Use match to handle if guess is not a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number! Please make sure you enter a number!\n");
                continue;
            },
        };

        println!("You guessed: {guess}");
        let delay = Duration::from_millis(200);

        slow_print("The number is........ ", delay);
        sleep(delay);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!\nYou win!");
                sleep(Duration::new(1000,0));
                return;
            },
        }
        println!();
        attempts += 1;

        if attempts == 4 {
            println!("One guess left! Make it count!")
        }
    }

    println!("You lose!\nThe correct number was {secret_number}");
    sleep(Duration::new(1000,0));
}