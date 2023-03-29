use std::collections::VecDeque;

pub fn queue() -> VecDeque<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue
}