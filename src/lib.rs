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
    match is_lead_vowel(&word) {
        true => format!("{}-yay", &word),
        false => format!("{}{}ay", &word[1..], first_letter(&word))
    }
}

fn first_letter(word: &String) -> String {
    word.chars().take(1).collect()
}

