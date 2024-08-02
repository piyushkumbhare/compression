use std::{collections::HashMap, hash::Hash};
use regex::Regex;
use super::utils::{format_radix, enumdup};

// Encode a regular string into a BWT string
pub fn encode(s: &str) -> String {
    // Create Suffix Array
    let mut sa: Vec<(usize, &str)> = (0..s.len())
        .map(|i| s.get(i..).unwrap())
        .enumerate()
        .collect();
    sa.sort_by_key(|f| f.1);
    let mut sa: Vec<usize> = sa.into_iter().map(|f| f.0).collect();
    sa.insert(0, s.len());

    let mut delim_pos: usize = 0;
    let mut encoded_string = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, pos) in sa.iter().enumerate() {
        if *pos > 0 {
            encoded_string.push(chars[pos - 1]);
        } else {
            delim_pos = i;
        }
    }
    encoded_string.insert_str(
        0,
        format!("{}|", format_radix(delim_pos as u32, 36)).as_str(),
    );
    encoded_string
}

// Decode a BWT string back into a regular string
pub fn decode(s: &str) -> String {
    let mut sorted = enumdup(bwt_to_tokens(s).0);
    let unsorted = enumdup(bwt_to_tokens(s).0);
    sorted.sort_by_key(|f| f.0.clone());

    let mut map: HashMap<(Token, usize), (Token, usize)> = HashMap::new();
    sorted.iter().zip(&unsorted).for_each(|f| {
        map.insert(f.1.clone(), f.0.clone());
    });
    let mut decoded_word: Vec<Token> = vec![];
    let mut current_char = &(Token::Delim, 0 as usize);
    while *decoded_word.last().unwrap_or(&Token::Char(' ')) != Token::Delim {
        let next_char = map.get(&current_char).unwrap();
        decoded_word.push(next_char.0.clone());
        current_char = next_char;
    }
    decoded_word.pop().unwrap();
    decoded_word
        .iter()
        .map(|t| match t {
            Token::Char(t) => *t,
            Token::Delim => ' ',
        })
        .collect()
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Token {
	Delim,
	Char(char),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Tokens(pub Vec<Token>);

// Parses a post-BWT string back into Tokens
fn bwt_to_tokens(string: &str) -> Tokens {
	let re = Regex::new(r"(\w+)\|([\w\W\n]*)").unwrap();
	let captures = re.captures(string).unwrap();

	let delim_pos: usize = usize::from_str_radix(&captures[1], 36).unwrap();
	let string = &captures[2];

	let mut tokens: Vec<Token> = string.chars().map(|c| Token::Char(c)).collect();
	tokens.insert(delim_pos, Token::Delim);
	Tokens(tokens)
}



