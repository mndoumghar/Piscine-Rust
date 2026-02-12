
use string_permutation::*;

fn main() {
    let word = "abcde";
    let word1 ="edgwa";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}

/* 
And its output

$ cargo run
Is "thought" a permutation of "thougth"? = true
$
 */