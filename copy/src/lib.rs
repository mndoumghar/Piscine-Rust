
pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c,(c as f64).exp(), ( c as f64).ln())
    
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for i in b {
        vec1.push(i);
        vec2.push((i as f64).exp());
    }
    (vec1, vec2)    

}
   
pub fn str_function(a: String) -> (String, String) {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let mut length : i32 = 0;
    for i in a.split(" ") {
        if length != a.len() as i32 {
        str1.push(' ');
        str2.push(' ');        }  
        str1.push_str(i);
        str2.push_str(&((i.parse::<f64>().expect("errrrr") as f64).exp().to_string())); 
        length+=1;     
    }
    
    (str1, str2)

}