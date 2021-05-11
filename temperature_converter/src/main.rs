// https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary

use std::io;

fn main() {
    println!("Temperature Converter\nSupports Celsius (C), Farenheit (F), Kelvin (K)");

    // Getting `temp`
    loop {
        println!("\nEnter a temperature:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input!");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid input!");
                continue;
            }
        };

        // Getting `from`
        loop {
            println!("\nEnter the original unit (C/F/K):");

            let mut from = String::new();

            io::stdin()
                .read_line(&mut from)
                .expect("Failed to read input!");
            let from = from.trim().to_lowercase();

            // Checking if input is valid
            if from != "c"
                && from != "celsius"
                && from != "f"
                && from != "farenheit"
                && from != "k"
                && from != "kelvin"
            {
                println!("\nInvalid input! Please type one of the following options:\n- `Celsius` or `C`\n- `Farenheit` or `F`\n- `Kelvin` or `K`\n(Not case sensitive)");
                continue;
            }

            // Getting `to`
            loop {
                println!("\nEnter your desired unit (C/F/K):");

                let mut to = String::new();
                io::stdin()
                    .read_line(&mut to)
                    .expect("Failed to read input!");
                let to = to.trim().to_lowercase();

                // Checking if input is valid
                if from != "c"
                    && from != "celsius"
                    && from != "f"
                    && from != "farenheit"
                    && from != "k"
                    && from != "kelvin"
                {
                    println!("\nInvalid input! Please type one of the following options:\n- `Celsius` or `C`\n- `Farenheit` or `F`\n- `Kelvin` or `K`\n(Not case sensitive)");
                    continue;
                }

                // Converting temperature
                // From Celsius
                if from == "c" || from == "celsius" {
                    // Celsius to Celsius
                    if to == "c" || to == "celsius" {
                        break println!("\nResult: {}", temp);
                    // Celsius to Farenheit
                    } else if to == "f" || to == "farenheit" {
                        break println!("\nResult: {}", temp * 9.0 / 5.0 + 32.0);
                    // Celsius to Kelvin
                    } else if to == "k" || to == "kelvin" {
                        break println!("\nResult: {}", temp + 273.15);
                    }
                // From Farenheit
                } else if from == "f" || from == "farenheit" {
                    // Farenheit to Celsius
                    if to == "c" || to == "celsius" {
                        break println!("\nResult: {}", (temp - 32.0) * 5.0 / 9.0);
                    // Farenheit to Farenheit
                    } else if to == "f" || to == "farenheit" {
                        break println!("\nResult: {}", temp);
                    // Farenheit to Kelvin
                    } else if to == "k" || to == "kelvin" {
                        break println!("\nResult: {}", (temp - 32.0) * 5.0 / 9.0 + 273.15);
                    }
                // From Kelvin
                } else if from == "k" || from == "kelvin" {
                    // Kelvin to Celsius
                    if to == "c" || to == "celsius" {
                        break println!("\nResult: {}", temp - 273.15);
                    // Kelvin to Farenheit
                    } else if to == "f" || to == "farenheit" {
                        break println!("\nResult: {}", (temp - 273.15) * 9.0 / 5.0 + 32.0);
                    // Kelvin to Kelvin
                    } else if to == "k" || to == "kelvin" {
                        break println!("\nResult: {}", temp);
                    }
                }
            }
            break;
        }
        break;
    }
    // Waits for a last input before closing the program
    println!("\nClick `Enter` to close...");
    let mut close = String::new();
    io::stdin()
        .read_line(&mut close)
        .expect("Failed to read input!");
}
