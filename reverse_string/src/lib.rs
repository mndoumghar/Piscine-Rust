
pub fn rev_str(input: &str) -> String {
     if  input.is_empty() {
        return "".to_string();
    }
    
    let mut res = String::new();
    let word: Vec<char> =input.chars().collect();
    let mut i : i32 = (word.len() -1 )as i32;
   
    loop {
        if i< 0 {
            break
        }
        res.push(word[i as usize]);
        i-=1;
    }

    res
   
}