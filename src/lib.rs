pub fn run(words: String) -> String {
    let words_list = words.split_whitespace();
    let res: Vec<String> = words_list
        .map(|x| parse_word(x.to_string()))
        .collect();
    res.join(" ")
}

fn is_lead_vowel(word: &String) -> bool {
    "aeiou".chars().any(|x| x.to_string() == first_letter(&word))
}

fn parse_word(word: String) -> String {
    if is_lead_vowel(&word) {
        format!("{}-yay", &word)
    } else {
        let first = first_letter(&word);
        format!("{}{}ay", &word[1..], first)
    }
}

fn first_letter(word: &String) -> String {
    word.chars().take(1).collect()
}

