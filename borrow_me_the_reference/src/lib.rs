pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch == '-' && i > 0 {
            result.pop();
            continue;
        }
        
        result.push(ch);
    }
    let mut result2 = String::new();

    for (i, ch) in result.chars().rev().enumerate() {
        if ch == '+' && i > 0 {
            result2.pop();
            continue;
        }
        result2.push(ch);
    }

    result2 = result2.chars().rev().collect::<String>();

    *s = result2
}

pub fn do_operations(v: &mut [String]) {
    for st in v {
        for (_, ch) in st.chars().enumerate() {
            if ch == '+' {
                let splitted = st.split('+').collect::<Vec<&str>>();
                let num1 = splitted[0].parse::<i32>().expect("errrr");
                let num2 = splitted[1].parse::<i32>().expect("errrr");
                let result = num1 + num2;
                *st = result.to_string();;break;
                } else if ch == '-' {
                let splitted: Vec<&str> = st.split('-').collect();
                let num1 = splitted[0].parse::<i32>().expect("errrr");
                let num2 = splitted[1].parse::<i32>().expect("errrr");
                let result = num1 - num2;
                *st = result.to_string();;break;   
                }               
            }
    }
 
}

