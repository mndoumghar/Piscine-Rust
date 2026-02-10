

pub fn first_subword(mut s: String) -> String {
    let (first,_) = s.split_at(5);

    first.to_string()
}