pub fn mean(list: &[i32]) -> f64 {
    let sum: f64 = list.iter().map(|&c| c as f64).sum();
    let div: f64 = list.len() as f64;
    sum / div
}

pub fn median(list: &[i32]) -> i32 {
let mut res: Vec<i32> = list.iter().map(|&c| c as i32).collect();
let  len = list.len() ;
    res.sort();
    println!("{:?}   \n {:?} ", res, list);
    if len % 2 != 0 {
        return res[len/2];
    }
    res[(len / 2) - 1]
}


use std::collections::HashMap;

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &value in list {
        *counts.entry(value).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode_val = list[0];
    for (&value, &count) in &counts {
        if count > max_count {
            max_count = count;
            mode_val = value;
        }
    }

    mode_val
}
