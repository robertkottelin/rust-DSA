pub fn vector(length: usize) -> Vec<i32> {
    let mut result = vec![0; length];
    for i in 0..length {
        result[i] = (i + 1) as i32;
    }
    result
}
