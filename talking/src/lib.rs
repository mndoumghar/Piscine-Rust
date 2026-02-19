pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    let mut checktouppper = true;
    for ch in text.split_whitespace() {
        let  first = ch.chars().next().expect("er");
        if first.is_lowercase() {
            checktouppper = false;
        }
    }
    let has_digit = text.chars().any(|c| c.is_ascii_digit());

    if checktouppper && text.trim_end().ends_with('?') && !has_digit {
        return "Quiet, I am thinking!";
    } 
    else if text.trim_end().ends_with('?') {
         return "Sure.";
    } else if checktouppper {
        return "There is no need to yell, calm down!";

    } else {
        "Interesting"
    }

}
