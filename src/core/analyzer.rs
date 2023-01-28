use super::super::common::definitions::Constant;
use super::super::common::definitions::PATTERNS;

fn analyze_lexem(lexem: &String) -> (Option<Constant>) {
    for (pattern, constant) in PATTERNS.iter() {
        let re = pattern.is_match(lexem);
        if re {
            return Some(*constant)
        }
    }
    None
}

fn get_tokens(lexems: Vec<String>) -> Vec<Constant> {
    let mut constants: Vec<Constant> = vec![];
    for lexem in lexems.iter() {
        match analyze_lexem(lexem) {
            Some(x) => {
                dbg!(&x);
                constants.push(x);
            },
            None => panic!("No regex could match lexem.")
        }
    }
    constants
}

pub fn group_lexems(content: &str) -> Vec<String> {
    // let mut aux_lexem = String::from("");
    let mut lexems: Vec<String> = Vec::new();
    let mut initial_index: usize = 0;
    let mut middle_of_lexem = false;

    let num_elements = content.chars().count();
    let content_chars: Vec<char> = content.chars().collect();

    let single_keywords: Vec<char> = vec!['(', ')', '{', '}', ';'];

    for (i, char) in content_chars.iter().enumerate() {
        // dbg!(char);
        if !middle_of_lexem && char.is_alphanumeric() {
            initial_index = i;
            middle_of_lexem = true;
        }
        if *char == ' ' || i == num_elements - 1 || single_keywords.contains(char) {
            middle_of_lexem = false;
        }
        if !middle_of_lexem {
            // dbg!(content_chars[initial_index..i+1].iter());
            let mut index = i;
            if i == num_elements - 1 {
                index = i + 1;
            }
            let constructed_lexem = content_chars[initial_index..index].iter().collect::<String>().trim().to_string();

            if constructed_lexem != "" {
                lexems.push(constructed_lexem);
            }
            initial_index = i;
        }
    }
    lexems
}

