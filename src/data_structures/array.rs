//Rust does not support using non-constant values for array sizes, using Vec instead
pub fn array(length: usize) -> Vec<i32> {
    let mut result = vec![0; length];
    for i in 0..length {
        result[i] = (i + 1) as i32;
    }
    let numbers = [1, 2, 3, 4, 5]; //arrays can be sliced
    let slice = &numbers[1..4];
    result
}
