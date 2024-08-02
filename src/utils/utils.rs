use std::{collections::HashMap, hash::Hash};


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
