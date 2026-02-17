pub fn is_pangram(s: &str) -> bool {
    let mut res1 = [false; 26];
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            res1[index] = true;
        }
    }

    res1.iter().all(|&seen| seen)
}
