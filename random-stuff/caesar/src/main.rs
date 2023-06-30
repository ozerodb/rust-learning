use clap::Parser;

/// Simple program to encrypt or decrypt a message using the Caesar cipher
#[derive(Parser, Debug)]
struct Args {
    /// The shift to use in the cipher
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..=26))]
    shift: u8,

    /// If specified, decrypt instead of encrypt
    #[arg(short, long, action)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long, num_args(1..), required=true)]
    message: Vec<String>,
}

fn caesar(input: &str, shift: u8, decrypt: bool) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                if decrypt {
                    (base + (c as u8 + 26 - shift - base) % 26) as char
                } else {
                    (base + (c as u8 + shift - base) % 26) as char
                }
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    if args.decrypt {
        println!("DECRYPTED:")
    } else {
        println!("ENCRYPTED:")
    }
    println!(
        "{}",
        caesar(&args.message.join(" "), args.shift, args.decrypt)
    );
}
