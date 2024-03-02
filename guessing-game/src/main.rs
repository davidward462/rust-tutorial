// Input/output library, random library, and comparison library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Guess the number");

    // generate random number between 1 and 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop
    {
        println!("Please input your guess.");

        // bind a new empty string to the variable 'guess'
        let mut guess = String::new();

        // get user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() 
        {
            // check if parse got a value that is can convert into an integer
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // check if input matches guess
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => 
            {
                println!("You win!");
                break;
                // end loop
            }

        }
    }

}
