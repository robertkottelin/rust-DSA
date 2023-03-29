use std::marker::Tuple;

pub fn tuples() -> Tuple{
    let tuple = (1, "hello", true);
    let (a, b, c) = tuple;
    tuple
}