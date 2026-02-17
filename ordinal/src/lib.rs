pub fn num_to_ordinal(x: u32) -> String {
    let res = match (x % 100) as u32 {
        11 | 12 | 13 | 14 => "th",
        _ =>
            match x % (10 as u32) {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
    };
    x.to_string() + &res.to_string()
}
