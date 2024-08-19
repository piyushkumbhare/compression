use std::{char, collections::HashMap, fmt::Display, usize};

use crate::utils::{
    encoder_trait::Encoder,
    utils::{format_radix, get_least_used_char, MAX_ASCII, MIN_ASCII},
};

/*
    This is the Run Length Encoding (RLE) module.

    Quite simply put, it replaces repeated consecutive characters with a count+char string

    A naiive approach to this algorithm may replace the string "boooook" with "b5ok", which can
    be decompressed correctly. However, a string like "b00000k" will become "b50k", which would be
    decompressed to "b000000000000000000000...k".

    My solution to this uses a delimeter approach, where each repeated sequence of characters will
    be replaced with (DELIM)(NUM)(CHAR). This way, I can code a parser to watch out for these (DELIM)
    characters, so it knows when to decompress a character and when to ignore it.

    To go a step further, instead of using a backslash '\', my code does an initial scan of the text to
    find the least used ASCII character and use IT as a delimeter to minimize the number of delimeter
    escapes needed. When I say "least used", an unused ASCII char is always chosen over a char with a
    non-zero count. And the reason I limit to ASCII chars and not UTF-8 is simply because ASCII is limited
    to 1 byte.

*/

// Upper & lower bounds for number of consecutive characters that induce an RLE replacement
// Lower bound = 4 because aaaa -> (DELIM)4a    4 bytes -> 3 bytes. Saves at least 1 byte
const MIN_REPEAT_COUNT: usize = 4;
// Upper bound = 36 because count is encoded as a usize in base-36
const MAX_REPEAT_COUNT: usize = 36;

pub struct RLE;

impl Encoder for RLE {
    fn encode(&self, s: &str) -> String {
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
                    if next != curr || count >= MAX_REPEAT_COUNT {
                        if MIN_REPEAT_COUNT < count {
                            encoded_string.push_str(
                                format!("{delim}{}{curr}", format_radix(count as u32 - 1, 36))
                                    .as_str(),
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
                    if MIN_REPEAT_COUNT < count {
                        encoded_string.push_str(
                            format!("{delim}{}{curr}", format_radix(count as u32 - 1, 36)).as_str(),
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

    fn decode(&self, s: &str) -> String {
        // TODO: get rid of these unwraps and make the whole compression chain safe
        let delim = s.chars().nth(0).unwrap();
        let s = s.get(1..).unwrap();

        let mut decoded_string = String::new();

        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.next() {
                    decoded_string.push(next);
                }
            } else if c == delim {
                let count_str = chars.next().unwrap().to_string();
                let count = usize::from_str_radix(&count_str, 36).unwrap() + 1;
                let char_to_repeat = chars.next().unwrap();
                decoded_string.push_str(char_to_repeat.to_string().repeat(count).as_str());
            } else {
                decoded_string.push(c);
            }
        }

        decoded_string
    }
}

/*
    MY NOTES ON THE TOPIC:


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


    NEW IDEA:
        Use only THREE characters per repeated sequence by replacing all repeates occurences with:
            (DELIM)(NUM)(CHAR)
            catdadaaaabobby -> catdad(DELIM)4abobby
        Write NUM as a single digit in base-36. This saves space at the cost of max repeat length
        If we read a RL of more than 36 characters, just max out each replacement at 36.
        aaaaa (40 a's) -> (DELIM)za(DELIM)4a        40 -> 6 bytes, still good compression.
        Besides, it's not very common to see lengths of same-chars over 36 chars long.
*/
