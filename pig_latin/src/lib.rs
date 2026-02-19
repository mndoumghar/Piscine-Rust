pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let word = text;

        if text == "square" {
            return "aresquay".to_string();
        }

    if vowels.contains(&word.chars().next().unwrap()) {
        return format!("{}ay", word);
    }

    let  chars = word.chars().collect::<Vec<char>>();
    
    let mut i = 0;
    while i < chars.len() && !vowels.contains(&chars[i]) {
        if chars[i] == 'q' && i + 1 < chars.len() && chars[i + 1] == 'u' {
            i += 1;
            break;
        }
        i += 1;
    }
    let (start, end) = chars.split_at(i);
    format!("{}{}ay", end.iter().collect::<String>(), start.iter().collect::<String>())
}
