use regex::Regex;
use std::{collections::HashMap, hash::Hash};

use crate::utils::{
    encoder_trait::Encoder,
    utils::{enumdup, format_radix},
};

/*
    This module contains code for the Burrows-Wheeler Transform.

    In essence, the BWT shuffles a string around such that long sequences of same-characters
    are common. This makes it prime for compression via tools like Run Length Encoding.

    In order to work, the BWT algorithm needs to add an end-of-string character (typically a $).
    There were a few appraoches I could have gone with here:

        1. Choose a hard-coded delimeter character (such as '\') to mark the end of the string. (GARBAGE)
            This requires me to replace already existing occurences of this delim with another
            '\' in order to avoid conflicts. However, this can bloat the string if the chosen delim
            is commonly used in the original string

        2. Use a dynamically-chosen delimeter character (PROMISING)
            This approach would pre-scan the string to find the least-used (or an unused) ASCII character
            to use as the delimeter. This would minimize the amount of replacements needed.
            However, if a string happens to use all ASCII characters, it would bloat the string a bit.

        3. Don't keep an actual delimeter, just keep track of WHERE it is! (CURRENT APPROACH)
            Instead of appending a physical delimeter to the end of the string, we can mathematically
            just keep track of WHERE it is in the string after the shuffle.
            To do this, we prepend "(POS)|" to the output string, so a "38|" tells the decoder
            that the delimeter is at position 38 in the true String.
            The benefit of this is that it sidesteps the hassle of escaping delimeters & bloating the string.
            And to keep things as small as possible, the position is encoded in base-36 instead of base-10.
*/

pub struct BWT;

impl Encoder for BWT {
    // Encode a regular string into a BWT string
    fn encode(&self, s: &str) -> String {
        // Create Suffix Array
        let mut sa: Vec<(usize, &str)> =
            s.char_indices().map(|(i, _)| &s[i..]).enumerate().collect();

        sa.sort_by_key(|f| f.1);
        let mut sa: Vec<usize> = sa.into_iter().map(|f| f.0).collect();
        sa.insert(0, sa.len());

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
        println!(
            "Placing delim at position {delim_pos} = {}",
            format_radix(delim_pos as u32, 36)
        );
        encoded_string.insert_str(
            0,
            format!("{}|", format_radix(delim_pos as u32, 36)).as_str(),
        );
        encoded_string
    }

    // Decode a BWT string back into a regular string
    fn decode(&self, s: &str) -> String {
        let mut sorted = enumdup(bwt_to_tokens(s).0);
        let unsorted = sorted.clone();
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
}

// This token struct is COMPLETELY separate from the one in tokens.rs
// This is used only within this file in order to avoid using a phsyical delimeter character.
// (credits to my younger sibling, Aadish, for this approach)

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Token {
    Delim,
    Char(char),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Tokens(Vec<Token>);

// Parses a post-BWT string back into Tokens
fn bwt_to_tokens(string: &str) -> Tokens {
    let re = Regex::new(r"(\w+)\|([\w\W\n]*)").unwrap();
    let captures = re.captures(string).unwrap();

    let delim_pos: usize = usize::from_str_radix(&captures[1], 36).unwrap();
    let string = &captures[2];

    println!("Delim Positon {} -> {delim_pos}", &captures[1]);
    let mut tokens: Vec<Token> = string.chars().map(|c| Token::Char(c)).collect();
    tokens.insert(delim_pos, Token::Delim);
    Tokens(tokens)
}
