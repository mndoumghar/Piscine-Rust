
use string_permutation::*;

fn main() {
    let word = "hello♥";
    let word1 = "♥oelhl";

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