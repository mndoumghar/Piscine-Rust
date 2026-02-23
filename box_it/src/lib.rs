pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s
    .split_whitespace() 
    .map(|ch| {
        if let Some(result) =  ch.strip_suffix("k")  {
            (result.parse::<f32>().unwrap() * 1000.0 ) as u32
        } else {
            ch.parse::<u32>().unwrap() 
        }
    })
    .map(Box::new).collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|num| *num).collect()

}


