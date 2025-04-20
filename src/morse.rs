use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MORSE_CODE: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('A', ".-");
        m.insert('B', "-...");
        m.insert('C', "-.-.");
        m.insert('D', "-..");
        m.insert('E', ".");
        m.insert('F', "..-.");
        m.insert('G', "--.");
        m.insert('H', "....");
        m.insert('I', "..");
        m.insert('J', ".---");
        m.insert('K', "-.-");
        m.insert('L', ".-..");
        m.insert('M', "--");
        m.insert('N', "-.");
        m.insert('O', "---");
        m.insert('P', ".--.");
        m.insert('Q', "--.-");
        m.insert('R', ".-.");
        m.insert('S', "...");
        m.insert('T', "-");
        m.insert('U', "..-");
        m.insert('V', "...-");
        m.insert('W', ".--");
        m.insert('X', "-..-");
        m.insert('Y', "-.--");
        m.insert('Z', "--..");

        // Numbers
        m.insert('0', "-----");
        m.insert('1', ".----");
        m.insert('2', "..---");
        m.insert('3', "...--");
        m.insert('4', "....-");
        m.insert('5', ".....");
        m.insert('6', "-....");
        m.insert('7', "--...");
        m.insert('8', "---..");
        m.insert('9', "----.");

        // Misc
        m.insert('.',".-.-.-");
        m.insert(',',"--..--");
        m.insert('?',"..--..");
        m.insert(';',"-.-.-.");
        m.insert(':',"---...");
        m.insert('-',"-....-");
        m.insert('/',"-..-.");
        m.insert('\'',".----.");
        m.insert('"',".-..-.");
        m.insert('(',"-.--.");
        m.insert(')',"-.--.-");
        m.insert('!',"-.-.--");

        m
    };

    pub static ref REVERSE_MORSE_CODE: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        for (k, v) in MORSE_CODE.iter() {
            m.insert(*v, *k);
        }
        m
    };
}

// Text to Morse with '/' word separation
pub fn text_to_morse(input: &str) -> String {
    input.to_uppercase()
        .split_whitespace()
        .map(process_word)
        .collect::<Vec<_>>()
        .join(" / ")
}

// Morse to Text translation
pub fn morse_to_text(input: &str) -> String {
    input.split('/')
        .map(|word| word.split_whitespace()
             .filter_map(|code| REVERSE_MORSE_CODE.get(code))
             .collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

fn process_word(word: &str) -> String {
    word.chars()
        .filter_map(|c| MORSE_CODE.get(&c))
        .map(|code| *code)
        .collect::<Vec<_>>()
        .join(" ")
}
