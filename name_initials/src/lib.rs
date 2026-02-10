
pub fn initials(names: Vec<&str>) -> Vec<String> {
    names  
    .iter()
    .map(|word| 
        {
            let mut res : String = String::new();
            let mut count: i32 = 0;

            for c in word.split_whitespace() {
                 if count>0 {
                    res.push_str(" ");
                }  
              
                res.push(c.chars().next().expect("hiy li bghat"));
                res.push('.');
                
                count+= 1;
            }
            res
    })
    .collect()
}
