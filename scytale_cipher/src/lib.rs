pub fn scytale_cipher(message: &str, letters_per_turn: usize) -> String {
    if message.is_empty() || letters_per_turn == 0 {
        return message.to_string();
    }

    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();

    if len < letters_per_turn {
        return message.to_string();
    }

    let mut result = String::new();
    let rows = letters_per_turn;
    let cols = (len + rows - 1) / rows; 

    for row in 0..rows {
        for col in 0..cols {
            let index = col * rows + row;
            if index < len {
                result.push(chars[index]);
            }
        }
    }

    result
}
