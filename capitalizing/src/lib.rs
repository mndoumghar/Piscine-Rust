pub fn capitalize_first(input: &str) -> String {
    let mut chas = input.chars();
    match chas.next() {
        Some(res) => res.to_uppercase().collect::<String>() + chas.as_str(),
        _ => String::new(),
    }
}

pub fn tollwercase_firsword(input: &str) -> String {
    let mut chas = input.chars();
    match chas.next() {
        Some(res) => res.to_lowercase().collect::<String>() + chas.as_str(),
        _ => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|ch| capitalize_first(ch))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            word.chars()
                .map(|ch| (
                    if !ch.is_uppercase() {
                        ch.to_uppercase().collect::<String>()
                    } else {
                        ch.to_lowercase().collect::<String>()
                    }
                ))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
