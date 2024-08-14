use std::{collections::HashMap, hash::Hash};

/*
    This is a utlility file which contains various helper-functions used throughout this project.

    Everything here is documented with a breif description of what it does.
*/

// Enumerates duplicates within a Vec, count starting at 0
pub fn enumdup<T>(v: Vec<T>) -> Vec<(T, usize)>
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

// Helper function to convert a u32 in base-10 to a different base (usually base-36)
pub fn format_radix(mut x: u32, radix: u32) -> String {
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

// Corresponding u32 values for the min & max ASCII values used in RLE.
pub const MAX_ASCII: u32 = 126;
pub const MIN_ASCII: u32 = 33;

// Retrieves the least used character in a string. An unused character will be returned if possible.
pub fn get_least_used_char(s: &str) -> char {
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
