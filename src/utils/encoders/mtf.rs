use regex::Regex;

use crate::utils::encoder_trait::Encoder;

/*
    This MTF Encoder is based off of an Adaptive-MTF algorithm by Brandon Simmons.
    http://brandon.si/code/an-adaptive-move-to-front-algorithm/

    Instead of a traditional MTF, where a known "alphabet" is decided beforehand (i.e. ASCII 0..128),
    this approach dynamically builds the alphabet in order to achieve better compression.

    The reason this adaptive appraoch can perform better is because the ideal "alphabet" for MTF is one where the
    most common characters are near the front of the list, which results in an encoded list
    with long strings of single-digit numbers.

    This comes at the cost of including the alphabet in the file itself so it can be decoded.
    However, the true size cost of this "key" is only the number of unique characters in the
    orginal string, meaning it is usually upper-bounded by 256.



    Code-wise, the problem with this current appraoch is that it encodes a string into another string.
    This is bad because it needs a space/delim between each integer, taking up more space.
    This results in nearly a -100% compression rate.

    A potential solution to this is to map from &str -> (String, Vec<usize>).
    The tuple is needed to pass along the unique dictionary generated through the
    Adaptive version of MTF. However, the Vec<usize> would allow some optimization when passing along to an RLE encoder

    But... this raises the inconvenience that this Tuple return value makes MTF's encode incompatible
    with the compression pipeline macro since it expects &dyn Fn(&str) -> String.
    And I really like the pipeline/macro I made... :(
*/

pub struct MTF;

impl Encoder for MTF {
    fn encode(&self, s: &str) -> String {
        let mut alphabet: Vec<char> = vec![];
        let mut output = String::new();
        let mut zero_count: usize = 0;
        let s_chars_len = s.chars().collect::<Vec<char>>().len();
        s.chars()
            .enumerate()
            .for_each(|(i, c)| match index_of(&alphabet, &c) {
                Some(index) => {
                    alphabet.remove(index);
                    alphabet.insert(0, c);

                    if index == 0 {
                        zero_count += 1;
                        if i == s_chars_len - 1 {
                            output.push_str(format!("{zero_count}z").as_str());
                        }
                        return;
                    }

                    if zero_count > 0 {
                        output.push_str(format!("{zero_count}z").as_str());
                        zero_count = 0;
                    }

                    output.push_str(format!("{index} ").as_str());
                }
                None => {
                    alphabet.insert(0, c);
                    if zero_count > 0 {
                        output.push_str(format!("{zero_count}z").as_str());
                        zero_count = 0;
                    }
                    output.push_str(format!("{} ", alphabet.len() - 1).as_str());
                }
            });
        let legend = format!(
            "{}{}",
            alphabet.iter().collect::<String>().as_str(),
            alphabet.first().unwrap()
        );
        output.insert_str(0, &legend);
        output
    }

    fn decode(&self, s: &str) -> String {
        let mut alphabet: Vec<char> = vec![];
        let mut output: Vec<char> = vec![];
        let mut string = "";
        for (i, c) in s.char_indices() {
            if alphabet.len() > 1 && *alphabet.first().unwrap() == c {
                string = s.get(i + 1..).unwrap();
                break;
            }
            alphabet.push(c);
        }

        // Unfold all repeated (0 )s in the string
        let re = Regex::new("([(0-9)+]*)z").unwrap();
        let mut new_s = string.to_string();
        re.captures_iter(s).for_each(|c| {
            let zero_count = c.get(1).unwrap().as_str().parse().unwrap();
            let to_replace = format!(" {}", c.get(0).unwrap().as_str());
            let unfolded = format!(" {}", "0 ".repeat(zero_count));
            new_s = new_s.replace(&to_replace, &unfolded);
        });
        let string = new_s;
        let indices: Vec<usize> = string.split(" ").filter_map(|i| i.parse().ok()).collect();
        // println!("alphabet: {:?}", alphabet);
        // println!("indices: {:?}", indices);

        for &index in indices.iter().rev() {
            let head = alphabet.remove(0);
            alphabet.insert(index, head);
            output.push(head);
        }
        let ret = output.iter().rev().collect();
        ret
    }
}

fn index_of<T>(v: &Vec<T>, obj: &T) -> Option<usize>
where
    T: Eq,
{
    for index in 0..v.len() {
        if v[index] == *obj {
            return Some(index);
        }
    }
    None
}
