/// Compute the factorial of a number
fn factorial(n: u128) -> Option<u128> {
    if n == 0 {
        Some(1)
    } else {
        n.checked_mul(factorial(n - 1)?)
    }
}

/// Compute the factorial of numbers passed as arguments
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    for arg in &args[1..] {
        match arg.parse::<u128>() {
            Ok(a) => println!(
                "{}! = {}",
                a,
                match factorial(a) {
                    Some(result) => result.to_string(),
                    None => "Overflow".to_string(),
                }
            ),
            Err(e) => println!("{} skipped ({})", arg, e),
        }
    }
}
