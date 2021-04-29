use std::io;

fn main() {
    loop {
        println!("Which number? (Click `Enter` to exit!)");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read input!");
        if !num.trim().is_empty() {
            let num: u32 = match num.trim().parse() {
                Ok(num) => {
                    if 1 <= num && num <= 185 {
                        num
                    } else {
                        println!("Invalid range! Please input numbers between 1 and 185.");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Invalid input!");
                    continue;
                }
            };
            println!("{}", fib(num - 1));
        } else {
            break;
        }
    }
}

fn fib(n: u32) -> u128 {
    let mut x: (u128, u128) = (1, 1);
    for _ in 1..n {
        x = (x.1, x.0 + x.1)
    }
    x.1
}
