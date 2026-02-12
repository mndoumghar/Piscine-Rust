use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut map = HashMap::new();
    if s1.len() != s2.len() {
        return false;
    }

    for ch in s1.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    let mut map2 = HashMap::new();
    for ch in s2.chars() {
        *map2.entry(ch).or_insert(0) += 1;
    }

    println!("{:?} ", map);
    println!("{:?} ", map2);
    let mut count1 = 0;
    let mut count2 = 0;
    for value in map.values() {
        count1 += value;
    }

    for value in map2.values() {
        count2 += value;
    }

    count1 == count2
}
