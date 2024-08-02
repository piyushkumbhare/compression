/*
    This MTF Encoder is based off of an Adaptive-MTF algorithm by Brandon Simmons.
    http://brandon.si/code/an-adaptive-move-to-front-algorithm/

    Instead of a traditional MTF, where a known "alphabet" is decided beforehand,
    this approach dynamically builds the alphabet in order to achieve better compression.
    This comes at the cost of including the alphabet in the file itself so it can be decoded.
    The true "cost" of this "key" is only the number of unique characters in the orginal string,
    meaning it is upper-bounded by 256 usually.
*/

pub fn encode(s: &str) -> String {
    let mut alphabet: Vec<char> = vec![];
    let mut output = String::new();
    s.chars().for_each(|c| match index_of(&alphabet, &c) {
        Some(index) => {
            alphabet.remove(index);
            alphabet.insert(0, c);
            output.push_str(format!("{index} ").as_str());
        }
        None => {
            alphabet.insert(0, c);
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

pub fn decode(s: &str) -> String {
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
    let indices: Vec<usize> = string.split(" ").filter_map(|i| i.parse().ok()).collect();
    println!("alphabet: {:?}", alphabet);
    println!("indices: {:?}", indices);

    for &index in indices.iter().rev() {
        let head = alphabet.remove(0);
        alphabet.insert(index, head);
        output.push(head);
    }
    output.iter().rev().collect()
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
