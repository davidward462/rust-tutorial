// use libraries
use std::io;
use rand::Rng;

fn random_integer(min: u32, max: u32) -> u32
{
    rand::thread_rng().gen_range(min..=max)
}

fn main() {
    println!("Multiplication");
    
    // initialize
    let question_count = 5;
    let mut correct_answers = 0;

    for _i in 0..question_count
    {
        // generate question values
        let multiplier = random_integer(1, 12);
        let multiplicand = random_integer(1, 12);
        let product = multiplier * multiplicand;

        println!("{} x {} = ", multiplier, multiplicand);

        // initialize and get user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line.");

        let input_value: u32 = match user_input.trim().parse()
        {
            // check if parse got a value that is can convert into an integer
            Ok(num) => num,
            Err(_) => continue,
        };

        // check answer
        if input_value == product
        {
            println!("\tcorrect");
            correct_answers += 1;
        }
        else
        {
            println!("\tincorrect: {}\n", product);
        }
    }

    // print results
    println!("You got {} / {}", correct_answers, question_count);
}

