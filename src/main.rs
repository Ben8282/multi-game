use std::io;
use std::process::Command;
mod guess;
mod csv_sort;
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
            println!("exit to exit program");
            println!("p-manager is a simple password manager ");
            println!("guess is a simple guess the number game");
            println!("csv-sort is a simple csv sorting program in case for some reason you have a bunch of ranom numbers in a csv file and you want to sort them");
            continue;
        } else if input == "p-manager" {
            Command::new("python3")
                .arg("src/p-manger.py")
                .status()
                .expect("there was a problem running the password manger python program");
            break;
        } else if input == "guess" {
            guess::guess();
        } else if input == "csv-sort" {
            csv_sort::sort_csv();
        } else if input == "exit" {
            std::process::exit(0);
        } else {
            println!("please input proper program use --help to list all them");
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
