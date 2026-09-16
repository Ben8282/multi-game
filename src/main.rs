use std::io;
use std::process::Command;
mod guess;
mod csv_sort;
mod unit_convert;
mod quadratic;
fn main() {
    println!("welcome to the combied program");
    println!("use --help for all commands");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("there was a error reading your message");
        let input = input.trim();
        if input == "--help" {
            //add a big list of all the commands
            println!("type the name or number of the program you want to run");
            println!("exit to exit program");
            println!(" 1. pong is new version of pong use arow keys and ws to play");
            println!(" 2. next_prime is a simple next prime number checker");
            println!(" 3. walk-in-the-park is a simple text-based adventure game");
            println!(" 4. p-manager is a simple password manager that stores your passwords in a plain text file encrpyption is not implemented yet");
            println!(" 5. quadratic is a simple quadratic equation solver");
            println!(" 6. prime is a simple prime number checker");
            println!(" 7. guess is a simple guess the number game");
            println!(" 8. unit-convert is a simple unit Converter");
            println!(" 9. csv-sort is a simple csv sorting program in case for some reason you have a bunch of random numbers in a csv file and you want to sort them");
            continue;
        } else if input.to_lowercase() == "p-manager" || input == "4" {
            Command::new("python3")
                .arg("src/p-manger.py")
                .status()
                .expect("there was a problem running the password manger python program");
        } else if input.to_lowercase() == "guess" || input == "7" {
            guess::guess();
        } 
        else if input.to_lowercase() == "csv-sort" || input == "9" {
            csv_sort::sort_csv();
        } else if input.to_lowercase() == "exit" {
            std::process::exit(0);
        } else if input.to_lowercase() == "unit-convert" || input == "8" {
            unit_convert::unitconvert()
        } else if input.to_lowercase() == "quadratic" || input == "5" {
            quadratic::solve_quadratic();
        } else if input.to_lowercase() == "prime" || input == "6" {
            Command::new("python3")
                .arg("src/prime.py")
                .status()
                .expect("there was a problem running the prime number checker python program");
        } else if input.to_lowercase() == "walk-in-the-park" || input == "3" {
            Command::new("java")
                .arg("WalkInThePark")
                .status()
                .expect("there was a problem running the walk in the park game");
        } else if input.to_lowercase() == "pong" || input == "1" {
            Command::new("python3")
                .arg("src/pong.py")
                .status()
                .expect("there was a problem running the pong python program");
            break;
        } else if input.to_lowercase() == "next_prime" || input == "2" {
            Command::new("python3")
                .arg("src/next_prime.py")
                .status()
                .expect("there was a problem running the next prime number checker python program");
        } else {
            println!("invalid command");
            println!("use --help for all commands");
            continue;
        }
        println!("would you like to run something else");
        loop {
            println!("please input y/n");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let _ = input.to_lowercase();
            if input == "y" {
                println!("use --help if your stuck");
                break;
            } else if input == "n" {
                std::process::exit(0)
            }
        }
    }
}
