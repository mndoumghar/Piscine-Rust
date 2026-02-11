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
    
    let mut res = String::new();
    for (i, ph) in matrix.clone().into_iter().enumerate() {
        for ch in ph.chars() {
            if !ch.is_digit(10) {
                res.push(ch);
            }
        }

        if matrix.len() -1  != i {
            res.push(' ');
        }
    }

    res
}
