extern crate yarns;

pub fn run(words: String) -> String {
    let words_list = words.split_whitespace();
    let res: Vec<String> = words_list
        .map(|x| parse_word(x.to_string()))
        .collect();
    res.join(" ")
}

fn is_lead_vowel(word: &String) -> bool {
    let vowels = String::from("aeiou");
    let head = yarns::head(&word);
    yarns::contains(&vowels, &head)
}

fn parse_word(word: String) -> String {
    match is_lead_vowel(&word) {
        true => format!("{}-yay", &word),
        false => format!("{}{}ay", &word[1..], yarns::head(&word))
    }
}

