fn main() {
    let string1 = String::from("I like apples");

    let mut translated_string = String::new();
    for word in string1.split_whitespace() {
        let translated_word = translate_word_to_pig_latin(word);
        translated_string.push_str(&translated_word);
        translated_string.push_str(" ");
    }

    println!("{}", translated_string)
}

fn starts_with_vocal(word: &str) -> bool {
    let vocals = ['a', 'e', 'i', 'o', 'u'];

    let first_letter = match word.chars().nth(0) {
        Some(letter) => letter,
        None => return false,
    };

    vocals.contains(&first_letter)
}

fn translate_word_to_pig_latin(word: &str) -> String {
    let mut translated_word = word.to_string();
    if starts_with_vocal(&word) {
        return translated_word + "-hay";
    } else {
        let german_consonants: [char; 21] = [
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'y', 'z',
        ];

        let mut first_run = true;

        for (idx, char) in word.char_indices() {
            if first_run {
                first_run = false;
                translated_word.push_str("-");
            }
            if german_consonants.contains(&char) {
                translated_word.remove(idx);
                translated_word.push_str(&char.to_string());
            } else {
                translated_word.push_str("ay");
                break;
            }
        }

        return translated_word;
    }
}
