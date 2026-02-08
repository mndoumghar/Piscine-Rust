pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let x: f64 = 5.0;
    let y: f64 = 9.0;
    let z: f64 = 32.0;
     (f - z) * x/y
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let x: f64 = 5.0;
    let y: f64 = 9.0;
    let z: f64 = 32.0;
     (c * y/x) + z 
}