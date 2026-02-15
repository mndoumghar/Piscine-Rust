pub fn edit_distance(source: &str, target: &str) -> usize {
    let length_source = source.len();
    let length_target = target.len();

    let vec_source: Vec<char> = source.chars().collect();
    let vec_target: Vec<char> = target.chars().collect();

    let mut matrix = vec![vec![0; length_target + 1]; length_source + 1];

    for i in 0..=length_source {
        matrix[i][0] = i;
    }
    for j in 0..=length_target {
        matrix[0][j] = j;
    }

    for i in 1..=length_source {
        for j in 1..=length_target {
            let cost = if vec_source[i - 1] == vec_target[j - 1] { 0 } else { 1 };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,     
                    matrix[i][j - 1] + 1      
                ),
                matrix[i - 1][j - 1] + cost   
            );
        }
    }

    matrix[length_source][length_target]
}
