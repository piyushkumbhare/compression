use std::{collections::HashMap, hash::Hash};

use super::parser::{Token, Tokens};

pub struct BWT;

impl BWT {
    pub fn encode(s: &str) -> String {
        // Create Suffix Array
        let mut sa: Vec<(usize, &str)> = (0..s.len()).map(|i| s.get(i..).unwrap()).enumerate().collect();
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
        encoded_string.insert_str(0, format!("{}|", BWT::format_radix(delim_pos as u32, 36)).as_str());
        encoded_string
    }
    
    pub fn decode(s: &str) -> String {
        let mut d = BWT::enumdup(Tokens::from_bwt(s).0);
        let s = BWT::enumdup(Tokens::from_bwt(s).0);
        d.sort_by_key(|f| f.0.clone());
        
        let mut map: HashMap<(Token, usize), (Token, usize)> = HashMap::new();
        d.iter().zip(&s).for_each(|f| {
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
        decoded_word.iter().map(|t| {
            match t {
                Token::Char(t) => *t,
                Token::Delim => {' '},
            }
        }).collect()
    }
    
    // Enumerates duplicates within a Vec, count starting at 0
    fn enumdup<T>(v: Vec<T>) -> Vec<(T, usize)>
    where
        T: Eq + Hash + Clone,
    {
        let mut map: HashMap<T, usize> = HashMap::new();
        v.into_iter()
            .map(|f| {
                if map.contains_key(&f) {
                    *map.get_mut(&f).unwrap() += 1;
                } else {
                    map.insert(f.clone(), 0);
                }
                let count = *map.get(&f).unwrap();
                (f, count)
            })
            .collect()
    }
    
    fn format_radix(mut x: u32, radix: u32) -> String {
        let mut result = vec![];
    
        loop {
            let m = x % radix;
            x = x / radix;
    
            // will panic if you use a bad radix (< 2 or > 36).
            result.push(std::char::from_digit(m, radix).unwrap());
            if x == 0 {
                break;
            }
        }
        result.into_iter().rev().collect()
    }
    
}
