pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let idx = key as usize;
      array.get(idx).map(|index| *index as usize)

}
