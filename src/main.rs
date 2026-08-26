use std::io;
use std::process::Command;
mod guess;
fn main() {
    println!("welcome to the combied program");
    println!("use --help for all commands");
    let x = 1;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("there was a error reading your message");
        let input = input.trim();
        if input == "--help" { //add a big list of all the commands
            println!("exit to exit program");
            println!("p-manager is a simple password manager ");
            println!("guess is a simple guess the number game");
            continue;
        } else if input == "p-manager" {
            Command::new("python3")
            .arg("src/p-manger.py")
            .status()
            .expect("there was a problem running the password manger python program");
            break;
        }
        else if input == "guess"{
            guess::guess();
        }
        else if input == "exit"{
            std::process::exit(0);
        }
        else{
            println!("please input proper program use --help to list all them");
        }
        println!("would you like to run something else");
        loop{
        println!("please input y/n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let _ = input.to_lowercase();
        if input == "y"{
        println!("use --help if your stuck");
        break;
        }
        else if input == "n"{
            std::process::exit(0)
        }
    }
    }
}
