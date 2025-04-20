mod morse;
use std::io::{self, BufRead};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "dotdash",
    version = "1.0.0",
    author = "epiGnosko",
    about = "Translate text to Morse code and back. Use --decode to decode Morse."
)]
struct Cli {
    /// Decode Morse to text instead of encoding
    #[arg(short = 'd', long = "decode")]
    decode: bool,
}

fn main() {
    let cli = Cli::parse();

    // Example usage with your morse module:
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        let input = line.unwrap();
        let output = if cli.decode {
            morse::morse_to_text(&input)
        } else {
            morse::text_to_morse(&input)
        };
        println!("{}", output);
    }
}
