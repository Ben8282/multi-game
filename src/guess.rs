use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn guess() {
    println!("welcome to the guessing game");

    println!("there will be a number bettwen 1 and 100");

    println!("and your goal will be to guess it i will tell you if you are to low or to high");
    // uses the thread rng method to generate the random number
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100); // generates the secret number
    loop {
        println!("guess the number");
        println!("input your guess");
        let mut guess = String::new(); // define variable
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line"); //read user input
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a valid number");
                continue;
            }
        };
        println!("you guessed:{}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win congrats");
                break;
            }
        }
    }
}
