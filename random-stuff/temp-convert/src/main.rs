use clap::Parser;

/// Simple program to convert between temperature units
#[derive(Parser)]
struct Args {
    /// The temperature to convert
    #[arg(short, long)]
    temperature: f32,

    /// The unit to convert from
    #[arg(short, long, value_parser = ["c", "C", "f", "F"], default_value = "c")]
    unit: String,
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn main() {
    let args = Args::parse();

    match args.unit.as_str() {
        "c" | "C" => {
            println!(
                "{}°C --> {}°F",
                args.temperature,
                to_fahrenheit(args.temperature)
            );
        }
        "f" | "F" => {
            println!(
                "{}°F --> {}°C",
                args.temperature,
                to_celsius(args.temperature)
            );
        }
        _ => unreachable!(),
    }
}
