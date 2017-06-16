pub fn run(words: &String) -> &String {
    words
}

pub fn is_lead_vowel(word: &String) -> bool {
    let vowels = String::from("aeiou");
    let mut found = false;
    match word.chars().next() {
        None => panic!("Empty String!"),
        Some(x) => {
            for c in vowels.chars() {
                if c == x {
                    found = true
                    }
            }
        }
    }
    found
}