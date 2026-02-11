use arrays::*;

fn main() {
    let mut i: i32= 0;

    let mut a : [i32;10]= [0;10].map(|mut x| {
        i+=1;
        x+=i;
        x
      
    } );
     let b = [5;10]; 
    println!("{:?}",a);
    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));

    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );

}

// And its output:

// $ cargo run
// The sum of the elements in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] is 55
// The sum of the elements in [5, 5, 5, 5, 5, 5, 5, 5, 5, 5] is 50
// Array of 32 elements filled with 10 = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
// $