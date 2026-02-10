
fn main() {
    let  s1= String::from("helloWorld") ; // stocke heap data
    let mut  s2 =     & s1;

    s2.push_str("hhh") ;

    println!("s2: {}", s2);

    println!("s1: {}", s1);



}