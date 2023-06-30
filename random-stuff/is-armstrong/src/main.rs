use ansi_term::Colour::{Cyan, Green, Red};

/// Compute Armstrong sum of a number
fn compute_armstrong_sum(number: u128) -> u128 {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let len = digits.len() as u32;
    digits.iter().map(|d| d.pow(len)).sum::<u128>()
}

/// Check if numbers passed as arguments are Armstrong numbers
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    for arg in &args[1..] {
        match arg.parse::<u128>() {
            Ok(a) => {
                let sum = compute_armstrong_sum(a);
                let is_armstrong = sum == a;
                if is_armstrong {
                    println!(
                        "{}",
                        Green.bold().paint(format!("✔ {a} is a Armstrong number"))
                    );
                } else {
                    println!(
                        "{}",
                        Red.bold().paint(format!(
                            "✘ {a} is not a Armstrong number (its Armstrong sum is {sum})"
                        ))
                    );
                }
            }
            Err(e) => println!("{}", Cyan.bold().paint(format!("⚠ {arg} skipped ({e})"))),
        }
    }
}
