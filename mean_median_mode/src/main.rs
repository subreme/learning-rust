// https://doc.rust-lang.org/book/ch08-03-hash-maps.html?highlight=summary#summary

use std::collections::HashMap;
use std::io;

fn main() {
    println!("Mean Median Mode...");
    'main: loop {
        let nums: Vec<f64> = prompt();
        if nums.len() > 0 {
            println!("\nMean: {}", mean(&nums));
            println!("Median: {}", median(&nums));
            println!("Mode: {}", {
                let mut x = String::new();
                for num in mode(&nums) {
                    x += &num.to_string();
                    x += " ";
                }
                x
            });
            loop {
                println!("\nRun again?");
                let mut run_again = String::new();
                io::stdin()
                    .read_line(&mut run_again)
                    .expect("Failed to read input!");
                match run_again.trim().to_lowercase().as_str() {
                    "yes" | "y" => break,
                    "no" | "n" => break 'main,
                    _ => continue,
                };
            }
        } else {
            println!("\nPlease include at least one number!");
            continue;
        }
    }
}

fn prompt() -> Vec<f64> {
    return loop {
        println!("\nEnter a list of numbers:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        let mut nums: Vec<f64> = vec![];
        for num in input.trim().split_whitespace() {
            match num.parse::<f64>() {
                Ok(x) => nums.push(x),
                Err(_) => println!("\nError: `{}` is not a number", num),
            }
        }
        nums.sort_by(|a, b| a.partial_cmp(b).expect("Failed to sort numbers!"));
        break nums;
    };
}

fn mean(nums: &[f64]) -> f64 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums.iter().sum::<f64>() / nums.len() as f64
    }
}

fn median(nums: &[f64]) -> f64 {
    if nums.len() == 1 {
        nums[0]
    } else if nums.len() % 2 == 1 {
        nums[nums.len() / 2]
    } else {
        (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) / 2f64
    }
}

fn mode(nums: &[f64]) -> Vec<f64> {
    let mut freq = HashMap::new();
    for num in nums {
        *freq.entry(num.to_string()).or_insert(0) += 1;
    }
    let mut value = vec![];
    let mut count: u32 = 0;
    for (x, y) in freq {
        if y >= count {
            let num: f64 = match x.parse::<f64>() {
                Ok(x) => x,
                Err(_) => 0f64,
            };
            if y > count {
                value = vec![];
                value.push(num);
                count = y;
            } else {
                value.push(num);
            }
        }
    }
    value
}
