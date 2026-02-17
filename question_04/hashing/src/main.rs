use hashing::*;

fn main() {
    let v = [2, 1, 5, 2, 7, 4];

    println!("mean {}", mean(&v));
     println!("median {}", median(&v));
    // println!("mode {}", mode(&v));
}

// $ cargo run
// mean 3.857142857142857
// median 4
// mode 5
// $
