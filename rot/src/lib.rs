pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|ch| {
        if ch.is_ascii_lowercase() {
            let offset = ch as i8 - b'a' as i8;
            let new_offset = (offset + key).rem_euclid(26);
            (b'a' + new_offset as u8) as char
        } else if ch.is_ascii_uppercase() {
            let offset = ch as i8 - b'A' as i8;
            let new_offset = (offset + key).rem_euclid(26);
            (b'A' + new_offset as u8) as char
        } else {
            ch
        }
    }).collect()
}
