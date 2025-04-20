mod morse;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut args = std::env::args();
    
    let decode_mode = args.any(|arg| arg == "--decode");

    for line in stdin.lock().lines() {
        let input = line?;
        let output = if decode_mode {
            morse::morse_to_text(&input)
        } else {
            morse::text_to_morse(&input)
        };
        println!("{}", output);
    }
    
    Ok(())
}
