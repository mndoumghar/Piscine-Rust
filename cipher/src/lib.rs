#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    if original.len() == ciphered.len() {
        return Ok(());
    }
    else {
        Err(CipherError {
            expected: ciphered.to_string() 
        
        
        })
    }
    
}