use std::io;

struct Unit {
    name: &'static str,
    symbol: &'static str,
    factor: f64, // factor short for conversion factor realative to a kilogram
}

fn convert(value: f64, from: &Unit, to: &Unit) -> f64 {
    value * from.factor / to.factor
}

pub fn unitconvert() {
    loop {
        println!(
            "Welcome to the Unit Converter!

Choose a category:

1. Temperature
2. Length
3. Weight

Enter a number:"
        );
        let mut category = String::new();
        io::stdin()
            .read_line(&mut category)
            .expect("failed to read input");
        let category = category.trim();
        if category == "1" {
            println!(
                "
Welcome to the Temperature Converter!

1. Celsius → Fahrenheit
2. Fahrenheit → Celsius
3. Celsius → Kelvin
4. Kelvin → Celsius
5. Fahrenheit → Kelvin
6. Kelvin → Fahrenheit

Enter a number (1-6):
"
            );
            let convertnum: String;
            loop {
                let mut convertnum_input = String::new();
                io::stdin()
                    .read_line(&mut convertnum_input)
                    .expect("failed to read input");
                let convertnum_input = convertnum_input.trim();
                if matches!(convertnum_input, "1" | "2" | "3" | "4" | "5" | "6") {
                    convertnum = convertnum_input.to_string();
                    break;
                } else {
                    println!("please make sure to input a number bettween 1 and 6");
                }
            }
            let convertnum = convertnum.as_str();
            let orignal_value: f64;
            loop {
                println!("what number to convert");
                let mut numtoconvert = String::new();
                io::stdin()
                    .read_line(&mut numtoconvert)
                    .expect("failed to read input");
                let numtoconvert = numtoconvert.trim();
                match numtoconvert.parse::<f64>() {
                    Ok(numtoconvert) => {
                        orignal_value = numtoconvert;
                        break;
                    }
                    Err(_) => {
                        println!("you didnt enter a valid number");
                        println!("please enter a valid number");
                    }
                }
            }
            let mut numtoconvert = orignal_value;
            if convertnum == "1" {
                numtoconvert = numtoconvert * 1.8;
                numtoconvert = numtoconvert + 32.0;
                println!(
                    "{orignal_value} degrees Celsius is {} degrees Fahrenheit",
                    numtoconvert
                );
            } else if convertnum == "2" {
                numtoconvert = numtoconvert - 32.0;
                numtoconvert = numtoconvert / 1.8;
                println!(
                    "{} degrees Fahrenheit is {} degrees Celsius",
                    orignal_value, numtoconvert
                );
            } else if convertnum == "3" {
                numtoconvert = numtoconvert + 273.15;
                println!("{orignal_value} degrees Celsius is {numtoconvert} degrees Kelvin");
            } else if convertnum == "4" {
                numtoconvert = numtoconvert - 273.15;
                println!(
                    "{} degrees Kelvin is {} degrees Celsius",
                    orignal_value, numtoconvert
                );
            } else if convertnum == "5" {
                numtoconvert = numtoconvert - 32.0;
                numtoconvert = numtoconvert / 1.8;
                numtoconvert = numtoconvert + 273.15;
                println!(
                    "{} degrees Fahrenheit is {} degrees Kelvin",
                    orignal_value, numtoconvert
                );
            } else if convertnum == "6" {
                numtoconvert = numtoconvert - 273.15;
                numtoconvert = numtoconvert * 1.8;
                numtoconvert = numtoconvert + 32.0;
                println!(
                    "{} degrees Kelvin is {} degrees Fahrenheit",
                    orignal_value, numtoconvert
                );
            }
            println!("press enter to exit...");
            let mut exit = String::new();
            io::stdin().read_line(&mut exit).unwrap();
            break;
        } else if category == "2" {
            println!(
                "Welcome to the Length Converter!

Choose the unit you want to convert FROM:

1. Millimetres (mm)
2. Centimetres (cm)
3. Metres (m)
4. Kilometres (km)
5. Inches (in)
6. Feet (ft)
7. Yards (yd)
8. Miles (mi)

Enter a number (1-8):"
            );
            let from_unit: u8;
            loop {
                let mut convert_length = String::new();
                io::stdin()
                    .read_line(&mut convert_length)
                    .expect("failed to read input");
                match convert_length.trim().parse::<u8>() {
                    Ok(num) if (1..=8).contains(&num) => {
                        from_unit = num;
                        break;
                    }
                    _ => {
                        println!("please enter a number between 1 and 8");
                    }
                }
            }
            println!("what number would you like to convert");
            let convert_length_amount: f64;
            loop {
                let mut convertwhat = String::new();
                io::stdin().read_line(&mut convertwhat).unwrap();
                match convertwhat.trim().parse::<f64>() {
                    Ok(num) => {
                        convert_length_amount = num;
                        break;
                    }
                    Err(_) => println!("there was a problem please enter a valid number"),
                };
            }
            // turn whatever they typed into metres first
            let mut length = convert_length_amount;
            if from_unit == 1 {
                length = length / 1000.0;
            } else if from_unit == 2 {
                length = length / 100.0;
            } else if from_unit == 4 {
                length = length * 1000.0;
            } else if from_unit == 5 {
                length = length * 0.0254;
            } else if from_unit == 6 {
                length = length * 0.3048;
            } else if from_unit == 7 {
                length = length * 0.9144;
            } else if from_unit == 8 {
                length = length * 1609.344;
            }
            println!(
                "
Choose the unit you want to convert TO:

1. Millimetres (mm)
2. Centimetres (cm)
3. Metres (m)
4. Kilometres (km)
5. Inches (in)
6. Feet (ft)
7. Yards (yd)
8. Miles (mi)

Enter a number (1-8):
"
            );
            let to_unit: u8;

            loop {
                let mut convert_length = String::new();
                io::stdin()
                    .read_line(&mut convert_length)
                    .expect("failed to read input");
                match convert_length.trim().parse::<u8>() {
                    Ok(num) if (1..=8).contains(&num) => {
                        to_unit = num;
                        break;
                    }
                    _ => {
                        println!("please enter a number between 1 and 8");
                    }
                }
            }
            // now turn the metres into the unit they asked for
            if to_unit == 1 {
                length = length * 1000.0;
            } else if to_unit == 2 {
                length = length * 100.0;
            } else if to_unit == 4 {
                length = length / 1000.0;
            } else if to_unit == 5 {
                length = length / 0.0254;
            } else if to_unit == 6 {
                length = length / 0.3048;
            } else if to_unit == 7 {
                length = length / 0.9144;
            } else if to_unit == 8 {
                length = length / 1609.344;
            }
            let original_unit;
            if from_unit == 1 {
                original_unit = "Millimetres (mm)";
            } else if from_unit == 2 {
                original_unit = "Centimetres (cm)";
            } else if from_unit == 3 {
                original_unit = "Metres (m)";
            } else if from_unit == 4 {
                original_unit = "Kilometres (km)";
            } else if from_unit == 5 {
                original_unit = "Inches (in)";
            } else if from_unit == 6 {
                original_unit = "Feet (ft)";
            } else if from_unit == 7 {
                original_unit = "Yards (yd)";
            } else {
                original_unit = "Miles (mi)";
            }
            let new_unit;
            if to_unit == 1 {
                new_unit = "Millimetres (mm)";
            } else if to_unit == 2 {
                new_unit = "Centimetres (cm)";
            } else if to_unit == 3 {
                new_unit = "Metres (m)";
            } else if to_unit == 4 {
                new_unit = "Kilometres (km)";
            } else if to_unit == 5 {
                new_unit = "Inches (in)";
            } else if to_unit == 6 {
                new_unit = "Feet (ft)";
            } else if to_unit == 7 {
                new_unit = "Yards (yd)";
            } else {
                new_unit = "Miles (mi)";
            }
            println!("{convert_length_amount} {original_unit} is {length} {new_unit}");
            println!("press enter to exit...");
            let mut exit = String::new();
            io::stdin().read_line(&mut exit).unwrap();
            break;
        } else if category == "3" {
            let weight_units = [
                Unit {
                    name: "Electronvolt mass equivalent",
                    symbol: "eV/c²",
                    factor: 0.00000000000000000000000000178266192,
                },
                Unit {
                    name: "Dalton / unified atomic mass unit",
                    symbol: "Da / u",
                    factor: 0.00000000000000000000000000166053907,
                },
                Unit {
                    name: "Picogram",
                    symbol: "pg",
                    factor: 0.000000000000001,
                },
                Unit {
                    name: "Nanogram",
                    symbol: "ng",
                    factor: 0.000000000001,
                },
                Unit {
                    name: "Microgram",
                    symbol: "µg",
                    factor: 0.000000001,
                },
                Unit {
                    name: "Milligram",
                    symbol: "mg",
                    factor: 0.000001,
                },
                Unit {
                    name: "Centigram",
                    symbol: "cg",
                    factor: 0.00001,
                },
                Unit {
                    name: "Decigram",
                    symbol: "dg",
                    factor: 0.0001,
                },
                Unit {
                    name: "Gram",
                    symbol: "g",
                    factor: 0.001,
                },
                Unit {
                    name: "Decagram",
                    symbol: "dag",
                    factor: 0.01,
                },
                Unit {
                    name: "Hectogram",
                    symbol: "hg",
                    factor: 0.1,
                },
                Unit {
                    name: "Kilogram",
                    symbol: "kg",
                    factor: 1.0,
                },
                Unit {
                    name: "Metric quintal",
                    symbol: "q",
                    factor: 100.0,
                },
                Unit {
                    name: "Metric tonne",
                    symbol: "t",
                    factor: 1000.0,
                },
                Unit {
                    name: "Kilotonne",
                    symbol: "kt",
                    factor: 1000000.0,
                },
                Unit {
                    name: "Megatonne",
                    symbol: "Mt",
                    factor: 1000000000.0,
                },
                Unit {
                    name: "Grain",
                    symbol: "gr",
                    factor: 0.00006479891,
                },
                Unit {
                    name: "Avoirdupois dram",
                    symbol: "dr",
                    factor: 0.0017718451953125,
                },
                Unit {
                    name: "Avoirdupois ounce",
                    symbol: "oz",
                    factor: 0.028349523125,
                },
                Unit {
                    name: "Pound",
                    symbol: "lb",
                    factor: 0.45359237,
                },
                Unit {
                    name: "Stone",
                    symbol: "st",
                    factor: 6.35029318,
                },
                Unit {
                    name: "Imperial quarter",
                    symbol: "qr",
                    factor: 12.70058636,
                },
                Unit {
                    name: "US hundredweight",
                    symbol: "US cwt",
                    factor: 45.359237,
                },
                Unit {
                    name: "Imperial hundredweight",
                    symbol: "Imp cwt",
                    factor: 50.80234544,
                },
                Unit {
                    name: "US short ton",
                    symbol: "short ton",
                    factor: 907.18474,
                },
                Unit {
                    name: "Imperial long ton",
                    symbol: "long ton",
                    factor: 1016.0469088,
                },
                Unit {
                    name: "Pennyweight",
                    symbol: "dwt",
                    factor: 0.00155517384,
                },
                Unit {
                    name: "Troy ounce",
                    symbol: "ozt",
                    factor: 0.0311034768,
                },
                Unit {
                    name: "Troy pound",
                    symbol: "lb t",
                    factor: 0.3732417216,
                },
                Unit {
                    name: "Metric carat",
                    symbol: "ct",
                    factor: 0.0002,
                },
                Unit {
                    name: "Tola",
                    symbol: "tola",
                    factor: 0.0116638038,
                },
                Unit {
                    name: "Momme",
                    symbol: "momme",
                    factor: 0.00375,
                },
                Unit {
                    name: "Japanese kan",
                    symbol: "kan",
                    factor: 3.75,
                },
                Unit {
                    name: "Thai baht (mass)",
                    symbol: "baht",
                    factor: 0.015244,
                },
                Unit {
                    name: "Chinese jin",
                    symbol: "jin",
                    factor: 0.5,
                },
                Unit {
                    name: "Chinese liang",
                    symbol: "liang",
                    factor: 0.05,
                },
                Unit {
                    name: "Chinese qian",
                    symbol: "qian",
                    factor: 0.005,
                },
                Unit {
                    name: "Chinese fen",
                    symbol: "fen",
                    factor: 0.0005,
                },
            ];
            println!(
                "Welcome to the Weight Converter!

Choose the unit you want to convert FROM:
"
            );
            for (index, Unit) in weight_units.iter().enumerate() {
                println!("{}. {}", index + 1, Unit.name);
            }

            let amount_of_weight_units = weight_units.len();
            println!("Enter a number (1-{}):", amount_of_weight_units);

            let from_unit_weight: usize;
            loop {
                let mut convert_length = String::new();
                io::stdin()
                    .read_line(&mut convert_length)
                    .expect("failed to read input");
                match convert_length.trim().parse::<usize>() {
                    Ok(num) if (1..=amount_of_weight_units).contains(&num) => {
                        from_unit_weight = num;
                        break;
                    }
                    _ => {
                        println!(
                            "please enter a number between 1 and {}",
                            amount_of_weight_units
                        );
                    }
                }
            }

            let from_unit_weight_indexer = &weight_units[from_unit_weight - 1];

            println!("what number would you like to convert");
            let convert_weight_amount: f64;
            loop {
                let mut convertwhat_weight = String::new();
                io::stdin().read_line(&mut convertwhat_weight).unwrap();
                match convertwhat_weight.trim().parse::<f64>() {
                    Ok(num) => {
                        convert_weight_amount = num;
                        break;
                    }
                    Err(_) => println!("there was a problem please enter a valid number"),
                };
            }

            println!(
                "Choose what unit you want to convert TO:
            "
            );

            for (index, unit) in weight_units.iter().enumerate() {
                println!("{}. {}", index + 1, unit.name);
            }

            println!("Enter a number (1-{}):", amount_of_weight_units);

            let to_unit_weight: usize;
            loop {
                let mut convert_length = String::new();
                io::stdin()
                    .read_line(&mut convert_length)
                    .expect("failed to read input");
                match convert_length.trim().parse::<usize>() {
                    Ok(num) if (1..=amount_of_weight_units).contains(&num) => {
                        to_unit_weight = num;
                        break;
                    }
                    _ => {
                        println!(
                            "please enter a number between 1 and {}",
                            amount_of_weight_units
                        );
                    }
                }
            }

            let to_unit_weight_indexer = &weight_units[to_unit_weight - 1];
            
            let result = convert(convert_weight_amount,from_unit_weight_indexer,to_unit_weight_indexer);

            println!("{} {} is {} {}", convert_weight_amount,from_unit_weight_indexer.symbol,result,to_unit_weight_indexer.symbol);
            println!("press enter to exit...");
            let mut exit = String::new();
            io::stdin().read_line(&mut exit).unwrap();
            break;
        } else {
            println!("please input a number thats 1 or 2")
        }
    }
}