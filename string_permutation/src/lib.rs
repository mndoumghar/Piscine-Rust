use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut map = HashMap::new();
    if s1.chars().count() != s2.chars().count() {
        return false;
    }
    for ch in s1.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    let mut map2 = HashMap::new();
    for ch in s2.chars() {
        *map2.entry(ch).or_insert(0) += 1;
    }

    let mut count = 0;
    for (key1, value1) in &map {
        for (key2, value2) in &map2 {
            if key1 == key2 && value1 == value2 {
                count += value1;
                continue;
            }
        }
    }

    count == s1.chars().count()
}
