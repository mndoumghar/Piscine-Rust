pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).abs().exp(), (c as f64).abs().ln())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for i in b {
        vec1.push(i);
        vec2.push((i as f64).abs().exp());
    }
    (vec1, vec2)
}

pub fn str_function(a: String) -> (String, String) {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let  count: i32 = a
    .split(' ')
    .count()
    .try_into()
    .expect("errrrr");
        let mut c = 0;

    for i in a.split(" ") {
         c = c +1;
        str1.push_str(i);
        str2.push_str(
            &((i.parse::<f64>().expect("errrrr") as f64)
                .abs()
                .exp()
                .to_string()),
        );

        if count > c  {
            str1.push(' ');
            str2.push(' ');

        }  
           
        
    }

    (str1, str2)
}
