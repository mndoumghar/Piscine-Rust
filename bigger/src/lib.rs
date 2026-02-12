use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut count = -1111;
    for (_,v) in h {
        if v>count {
            count = v; 
        }
    }
    count

}
