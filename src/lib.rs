pub fn run(words: String) -> String {
    let words_list = words.split_whitespace();
    let res: Vec<String> = words_list.map(|x| handle(x.to_string())).collect();
    res.join(" ")
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

fn handle(word: String) -> String {
    if let true = is_lead_vowel(&word) {
        let temp = String::new();
        let mut temp = temp + &word;
        temp.push_str("-yay");
        temp
    } else {
        let mut temp = String::new();
        let first: String = word.chars().take(1).collect();
        temp.push_str(&word[1..]);
        let mut temp = temp + &first;
        temp.push_str("ay");
        temp
    }
}

