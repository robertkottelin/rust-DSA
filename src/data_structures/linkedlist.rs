// LinkedLists: A sequence of elements that are connected by pointers
use std::collections::LinkedList;

pub fn linkedlist() -> LinkedList<i32> {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list
}