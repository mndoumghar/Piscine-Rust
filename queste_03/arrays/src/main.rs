use arrays::*;

fn main() {
    let mut i: i32= 0;

    let  a = [0;10].map(|mut x| {
        i+=1;
        x+=i;
        x
      
    });
        
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thirtytwo_tens() {
        assert_eq!(thirtytwo_tens(), [10; 32]);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum((1..=10).collect::<Vec<_>>().as_slice()), (1..=10).sum());
    }
}