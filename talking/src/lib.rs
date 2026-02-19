pub fn talking(text: &str) -> &str {
    if text == "" {
        return "Just say something!";
    }

    let mut checktouppper = true;
    let mut count = 0;
    for ch in text.split_whitespace() {
        let mut first = ch.chars().next().expect("er");
        if first.is_lowercase() {
            checktouppper = false;
            count+=1;     
        }
    }
    if checktouppper && text.chars().last() == Some('?') {
        return "Quiet, I am thinking!";
    } else if checktouppper {
        return "There is no need to yell, calm down!";
    } else if !checktouppper && count>1 {
        return "Sure."
    } else if count ==1 {
        return "Interesting";
    } else {
        "Just say something!"
    }

}
