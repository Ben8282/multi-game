use std::io;
use std::process::Command;
mod guess;
fn main() {
    println!("welcome to the combied program");
    println!("use --help for all commands");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("there was a error reading your message");
        let input = input.trim();
        if input == "--help" { //add a big list of all the programs
            println!("p-manager is a simple password manager ");
            println!("guess is a simple guess the number game");
            continue;
        } else if input == "p-manager" {
            Command::new("python3")
            .arg("p-manger.py")
            .status()
            .expect("there was a problem running the password manger python program");
            println!("p-manager has been run");
            break;
        }
        else if input == "guess"{
            guess::guess();
            break;
        }
    
    }
}
