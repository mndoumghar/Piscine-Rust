use std::io;
// use std::io::prelude::*;
// use std::fs::File;

fn main() {
    let mut strr = String::new();
    println!(
        "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
    );

    io::stdin().read_line(&mut strr).expect("Failed to read line");
    let str2 = "The letter e";
    let mut i = 0;
    loop {
        i += 1;
        strr.clear();
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        io::stdin().read_line(&mut strr).expect("Failed to read line");
        if str2.trim() == strr.trim() {
            println!("Number of trials: {}", i);
            break;
        }
        
    }
}
