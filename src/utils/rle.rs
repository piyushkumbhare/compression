use std::{char, collections::HashMap, fmt::Display, usize};

use crate::utils::utils::format_radix;

#[derive(Debug, Clone)]
pub struct RLE(pub String);

const MAX_COUNT: usize = 5;

const MAX_ASCII: u32 = 126;
const MIN_ASCII: u32 = 33;

pub fn encode(s: &str) -> String {
    let mut count: usize = 1;
    let delim = get_least_used_char(s);
    println!("Delim: ({delim}), ASCII: {}", u32::from(delim));
    let s = s.replace(r#"\"#, format!(r#"\\"#).as_str());
    let s = s.replace(delim, format!(r#"\{delim}"#).as_str());
    let mut chars = s.chars().peekable();
    let mut encoded_string = String::new();
    while let Some(curr) = chars.next() {
        match chars.peek() {
            Some(&next) => {
                if next != curr {
                    if count > MAX_COUNT {
                        encoded_string.push_str(
                            format!("{delim}{},{curr}", format_radix(count as u32, 36)).as_str(),
                        );
                    } else {
                        encoded_string.push_str(curr.to_string().repeat(count).as_str());
                    }
                    count = 1;
                } else {
                    count += 1;
                }
            }
            None => {
                if count > MAX_COUNT {
                    encoded_string.push_str(
                        format!("{delim}{},{curr}", format_radix(count as u32, 36)).as_str(),
                    );
                } else {
                    encoded_string.push_str(curr.to_string().repeat(count).as_str());
                }
            }
        }
    }
    encoded_string.insert(0, delim);
    encoded_string
}

pub fn decode(s: &str) -> Option<String> {
    let Some(delim) = s.chars().nth(0) else {
        return None;
    };

    let Some(s) = s.get(1..) else {
        return None;
    };

    println!("Delim: ({delim}), String: {s}");

    let mut decoded_string = String::new();

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                decoded_string.push(next);
            }
        } else if c == delim {
            let mut count_str = String::new();
            while let Some(count_char) = chars.next() {
                if count_char != ',' {
                    count_str.push(count_char);
                } else {
                    break;
                }
            }
            println!("Count: {count_str}");
            let count = usize::from_str_radix(&count_str, 36).unwrap();
            let char_to_repeat = chars.next().unwrap();
            decoded_string.push_str(char_to_repeat.to_string().repeat(count).as_str());
        } else {
            decoded_string.push(c);
        }
    }

    Some(decoded_string)
}

fn get_least_used_char(s: &str) -> char {
    let mut map: HashMap<char, usize> = HashMap::new();

    for num in MIN_ASCII..=MAX_ASCII {
        if let Some(c) = char::from_u32(num) {
            map.insert(c, 0);
        }
    }

    s.chars().for_each(|c| {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });

    map.iter()
        .filter(|(&k, _v)| k as u32 >= MIN_ASCII && k as u32 <= MAX_ASCII)
        .min_by_key(|x| x.1)
        .unwrap_or((&'\\', &0))
        .0
        .to_owned()
}

impl Display for RLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.as_str())
    }
}

/*
    Naive Idea:
        Direct replace repeated chars with usizechar
    Examples:
        aaaabbbbcccc -> 4a4b4c -> aaaabbbbcccc          Works, ideal case
        4444aaaabbbb -> 444a4b -> 444 x a + bbbb        Breaks on decode

    MY IDEA:
        (DELIM)usize,char(DELIM)usize,char...
        Delimeter character is chosen to be the least frequent ASCII char in the string
        To determine what the ASCII char is when decoding, the delim is pushed to the start of the string
        Only replace if 5+ of same char in a row -> Results in a worst case compression of -1 bytes
        Serialize usize as Base 36 to save even more chars

        In order to prevent problems with (DELIM) showing up in source text:
            1. Replace all \ with \\
            2. Add a \ before any occurance of the delim
            3. When decoding, text parser treats next char as a regular token if current char is \
    Examples:
        4444aaaabbbb -> 4,4 4,a 4,b -> original			Saves 1 byte lol, but safer
        aaaa bbbb cc -> 4,a   4,b   cc -> original      Saves -2 bytes, still consistent
        ,,,, ,,,, aa -> 4,,   4,,   aa -> original		Saves -2 bytes, still consistent
                  bb -> 10,  bb							Saves 6 bytes, still consistent

*/
