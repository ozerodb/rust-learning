/// Check if a string is a palindrome
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let input = &args[1..].join(" ");

    let reversed = input.chars().rev().collect::<String>();

    if input == &reversed {
        println!("'{}' is a palindrome", input);
    } else {
        println!("'{}' is not a palindrome - reversed '{}'", input, reversed);
    }
}
