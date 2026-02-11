pub fn arrange_phrase(phrase: &str) -> String {
    let mut place: Vec<i32> = Vec::new();
    let word_count = phrase.split_whitespace().count();
    let mut matrix: Vec<String> = vec![String::new(); word_count];

    for ph in phrase.split_whitespace() {
        for ch in ph.chars() {
            if let Some(digit) = ch.to_digit(10) {
                place.push(digit as i32);
            }
        }
    }
    place.sort();
    
    for ph in phrase.split_whitespace() {
        for ch in ph.chars() {
            if let Some(digit) = ch.to_digit(10) {
                for (i, &p) in place.iter().enumerate() {
                    if p == digit as i32 {
                        matrix[i] = ph.to_string();
                        break;
                    }
                }
            }
        }
    }

 

    matrix.join(" ").chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect()
}
