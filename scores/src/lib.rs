pub fn score(word: &str) -> u64 {
    let word = word.to_ascii_uppercase();
    let mut total = 0;
    for c in word.chars() {
        if "AEIOULNRST".contains(c) {
            total += 1;
        } else if "DG".contains(c) {
            total += 2;
        } else if "BCMP".contains(c) {
            total += 3;
        } else if "FHVWY".contains(c) {
            total += 4;
        } else if c == 'K' {
            total += 5;
        } else if "JX".contains(c) {
            total += 8;
        } else if "QZ".contains(c) {
            total += 10;
        }
    
    }

    total
}
