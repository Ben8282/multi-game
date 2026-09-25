use std::io;
use std::process::Command;
mod csv_sort;
mod guess;
mod quadratic;
mod unit_convert;
fn main() {
    println!("welcome to the combied program");
    println!("use --help for all commands");
    let python;
    if Command::new("python3").arg("--version").output().is_ok() {
        python = "python3";
    } else if Command::new("python").arg("--version").output().is_ok() {
        python = "python";
    } else {
        println!("python is not installed on your system");
        println!("please install python to use this program");
        std::process::exit(1);
    }
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("there was a error reading your message");
        let input: &str = input.trim();
        if input == "--help" {
            //add a big list of all the commands
            println!("type the name or number of the program you want to run");
            println!("exit to exit program");
            println!(" 1. pong is new version of pong use arow keys and ws to play");
            println!(" 2. next_prime is a simple next prime number checker");
            println!(" 3. walk-in-the-park is a simple text-based adventure game");
            println!(
                " 4. p-manager is a simple password manager that stores your passwords in a plain text file encrpyption is not implemented yet"
            );
            println!(" 5. quadratic is a simple quadratic equation solver");
            println!(" 6. prime is a simple prime number checker");
            println!(" 7. guess is a simple guess the number game");
            println!(" 8. unit-convert is a simple unit Converter");
            println!(
                " 9. csv-sort is a simple csv sorting program in case for some reason you have a bunch of random numbers in a csv file and you want to sort them"
            );
            continue;
        } else if input.to_lowercase() == "p-manager" || input == "4" {
            Command::new(python)
                .arg("src/p-manager.py")
                .status()
                .expect("there was a problem running the password manger python program");
        } else if input.to_lowercase() == "guess" || input == "7" {
            guess::guess();
        } else if input.to_lowercase() == "csv-sort" || input == "9" {
            csv_sort::sort_csv();
        } else if input.to_lowercase() == "exit" {
            std::process::exit(0);
        } else if input.to_lowercase() == "unit-convert" || input == "8" {
            unit_convert::unitconvert()
        } else if input.to_lowercase() == "quadratic" || input == "5" {
            quadratic::solve_quadratic();
        } else if input.to_lowercase() == "prime" || input == "6" {
            Command::new(python)
                .arg("src/prime.py")
                .status()
                .expect("there was a problem running the prime number checker python program");
        } else if input.to_lowercase() == "walk-in-the-park" || input == "3" {
            Command::new("java")
                .arg("src/Walk-in-the-Park.java")
                .status()
                .expect("there was a problem running the walk in the park game");
        } else if input.to_lowercase() == "pong" || input == "1" {
            Command::new(python)
                .arg("src/pong.py")
                .status()
                .expect("there was a problem running the pong python program");
        } else if input.to_lowercase() == "next_prime" || input == "2" {
            Command::new(python)
                .arg("src/nextprimemain.py")
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
            let input = input.to_lowercase();
            if input == "y" {
                println!("use --help if your stuck");
                break;
            } else if input == "n" {
                std::process::exit(0)
            }
        }
    }
}
