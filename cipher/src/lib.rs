#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let org: String = original
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                (b'Z' - ((c as u8) - b'A')) as char
            } else if c.is_ascii_lowercase() {
                (b'z' - ((c as u8) - b'a')) as char
            } else {
                c
            }
        })
        .collect();

    if org== ciphered {
        return Ok(());
    } else {
        Err(CipherError {
            expected: ciphered.to_string(),
        })
    }
}
