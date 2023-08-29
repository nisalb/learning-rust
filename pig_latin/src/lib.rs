pub fn encode(english: &str) -> String {
    let mut pig = String::from("");

    let mut in_word = english.starts_with(" ");
    let mut first_char = '.';

    for c in english.chars() {
        if c.is_whitespace() {
            if !in_word {
                // we are passing a set of whitespaces
                continue;
            } else {
                // the word we were processing has ended
                add_suffix(&mut pig, first_char, true);
                in_word = false;
            }
        } else {
            if !in_word {
                // we just stepped inside a word, save the first character
                if is_vowel(c) {
                    pig.push(c);
                }
                first_char = c;
                in_word = true;
            } else {
                pig.push(c);
            }
        }
    }

    if in_word {
        add_suffix(&mut pig, first_char, false);
    }

    pig
}

fn add_suffix(buffer: &mut String, first_char: char, add_space: bool) {
    let suffix = if is_vowel(first_char) {
        "hay".to_string()
    } else {
        format!("{first_char}ay")
    };

    let suffix = format!("-{suffix}{}", if add_space { " " } else { "" });
    buffer.push_str(&suffix);
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
