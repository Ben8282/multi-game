use bigdecimal::BigDecimal;
use std::fs::File;
use std::io;
use std::io::Write;
pub fn sort_csv() {
    println!("welcome to my sorting algorithm");
    println!("press 1 for ascending order or 2 for descending order");
    let choice = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_string();
        if choice == "1" {
            break choice;
        } else if choice == "2" {
            break choice;
        } else {
            println!("invalid choice, please try again");
            println!("press 1 for ascending order or 2 for descending order");
        }
    };
    if choice == "1" {
        println!("please enter the csv file path");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let file_path = input.trim().to_string();
        let file = File::open(&file_path).unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            //.delimiter(b',') optional, default is b','
            .from_reader(file);
        let mut records: Vec<BigDecimal> = Vec::new();
        for result in reader.records() {
            let record = result.unwrap();
            for field in record.iter() {
                let value: BigDecimal = field.parse::<BigDecimal>().unwrap();
                records.push(value);
            }
        }
        records.sort_unstable();
        println!(
            "what would you like to do with the sorted values?
            (1) print to console,
            (2) write to file
            (3) overwrite the original file"
        );
        loop {
        let mut input = String::new();
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice = input.trim();
            match choice {
                "1" => {
                    for value in &records {
                        println!("sorted: {value}");
                    }
                    break;
                }
                "2" => {
                    // Implementation for writing to file
                    println!("please enter the output file path");
                    let mut output_path = String::new();
                    io::stdin().read_line(&mut output_path)
                        .expect("Failed to read line");
                    let output_path = output_path.trim();
                    let mut file = File::create(output_path).unwrap();
                    for value in &records {
                        writeln!(file, "{value}").unwrap();
                    }
                    break;
                }
                "3" => {
                    // Implementation for overwriting original file
                    println!("overwriting original file");
                    let mut file = File::create(&file_path).unwrap();
                    for value in &records {
                    writeln!(file, "{value}").unwrap();
                    }
                    break;
                }
                _ => {
                    println!("Invalid choice");
                    println!(
                        "what would you like to do with the sorted values? 
            (1) print to console, 
            (2) write to file
            (3) overwrite the original file"
                    );
                    continue;
                }
            }
        }
    } else if choice == "2" {
        println!("please enter the csv file path");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let file_path = input.trim().to_string();
        let file = File::open(&file_path).unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            //.delimiter(b',') optional, default is b','
            .from_reader(file);
        let mut records: Vec<BigDecimal> = Vec::new();
        for result in reader.records() {
            let record = result.unwrap();
            for field in record.iter() {
                let value: BigDecimal = field.parse::<BigDecimal>().unwrap();
                records.push(value);
            }
        }
        records.sort_unstable_by(|a, b| b.cmp(a));
        println!(
            "what would you like to do with the sorted values? 
            (1) print to console, 
            (2) write to file
            (3) overwrite the original file"
        );
        loop {
            let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice = input.trim();
            match choice {
                "1" => {
                    for value in &records {
                        println!("sorted: {value}");
                    }
                    break;
                }
                "2" => {
                    // Implementation for writing to file
                    println!("please enter the output file path");
                    let mut output_path = String::new();
                    io::stdin().read_line(&mut output_path)
                        .expect("Failed to read line");
                    let output_path = output_path.trim();
                    let mut file = File::create(output_path).unwrap();
                    for value in &records {
                        writeln!(file, "{value}").unwrap();
                    }
                    break;
                }
                "3" => {
                    // Implementation for overwriting original file
                    println!("overwriting original file");
                    let mut file = File::create(&file_path).unwrap();
                    for value in &records {
                    writeln!(file, "{value}").unwrap();
                    }
                    break;
                }
                _ => {
                    println!("Invalid choice");
                    println!(
                        "what would you like to do with the sorted values? 
            (1) print to console, 
            (2) write to file
            (3) overwrite the original file"
                    );
                    continue;
                }
            }
        }
    }
}
