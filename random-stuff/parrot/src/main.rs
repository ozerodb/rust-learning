use std::io;
use std::io::*;

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        print!("🗣️ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.truncate(input.trim_end().len());
        println!("{input} 🦜\n");

        if input == "bye" {
            break;
        }
    }
}
