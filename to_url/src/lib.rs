pub fn to_url(s: &str) -> String {
    let mut res  = String::new();
    for ch in s.chars() {
        if ch == ' ' {
            res.push_str("%20");
            continue

        } 
        res.push_str(&ch.to_string());

    }
    
    res
}