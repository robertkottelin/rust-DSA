use std::collections::BinaryHeap;

pub fn binaryheap() -> BinaryHeap<i32> {
    let mut result = BinaryHeap::new();
    result.push(25);
    result.push(24);
    result.push(23);
    result.push(27);
    result
}