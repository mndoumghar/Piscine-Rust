pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().rposition(|&index| index==key)
}
