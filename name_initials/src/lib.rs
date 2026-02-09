
pub fn initials(names: Vec<&str>) -> Vec<String> {
    names  
    .iter()
    .map(|word| 
    {
            word.split_whitespace()
            .map(|c| c.chars().next().expect("").to_string() + ".").collect::<Vec<String>>().join(" ")
    })
    .collect()
}
