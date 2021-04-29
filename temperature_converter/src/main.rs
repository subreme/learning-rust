// https://doc.rust-lang.org/book/ch03-05-control-flow.html

use colored::*;
use std::io;

fn main() {
    println!(
        "{} {}\nSupports {} (C), {} (F), and {} (K)",
        "Temperature".red(),
        "Converter".blue(),
        "Celsius".green(),
        "Farenheit".yellow(),
        "Kelvin".magenta()
    );
    // Getting `temp`
    loop {
        println!("Enter a temperature:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input!");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        // Getting `from`
        loop {
            println!(
                "Enter the original unit ({}/{}/{}):",
                "C".green(),
                "F".yellow(),
                "K".magenta()
            );

            let mut from = String::new();

            io::stdin()
                .read_line(&mut from)
                .expect("Failed to read input!");
            let from = from.trim().to_lowercase();

            // Getting `to`
            loop {
                println!(
                    "Enter your desired unit ({}/{}/{}):",
                    "C".green(),
                    "F".yellow(),
                    "K".magenta()
                );

                let mut to = String::new();
                io::stdin()
                    .read_line(&mut to)
                    .expect("Failed to read input!");
                let to = to.trim().to_lowercase();

                // Converting temperature
                // From Celsius
                if from == "c" || from == "celsius" {
                    // Celsius to Celsius
                    if to == "c" || to == "celsius" {
                        break println!("Result: {}", temp);
                    // Celsius to Farenheit
                    } else if to == "f" || to == "farenheit" {
                        break println!("Result: {}", temp * 9.0 / 5.0 + 32.0);
                    // Celsius to Kelvin
                    } else if to == "k" || to == "kelvin" {
                        break println!("Result: {}", temp + 273.15);
                    }
                // From Farenheit
                } else if from == "f" || from == "farenheit" {
                    // Farenheit to Celsius
                    if to == "c" || to == "celsius" {
                        break println!("Result: {}", (temp - 32.0) * 5.0 / 9.0);
                    // Farenheit to Farenheit
                    } else if to == "f" || to == "farenheit" {
                        break println!("Result: {}", temp);
                    // Farenheit to Kelvin
                    } else if to == "k" || to == "kelvin" {
                        break println!("Result: {}", (temp - 32.0) * 5.0 / 9.0 + 273.15);
                    }
                // From Kelvin
                } else if from == "k" || from == "kelvin" {
                    // Kelvin to Celsius
                    if to == "c" || to == "celsius" {
                        break println!("Result: {}", temp - 273.15);
                    // Kelvin to Farenheit
                    } else if to == "f" || to == "farenheit" {
                        break println!("Result: {}", (temp - 273.15) * 9.0 / 5.0 + 32.0);
                    // Kelvin to Kelvin
                    } else if to == "k" || to == "kelvin" {
                        break println!("Result: {}", temp);
                    }
                }
            }
            break;
        }
        break;
    }
}
