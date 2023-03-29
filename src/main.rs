use std::collections::linked_list;

mod data_structures;

fn main() {
    let getarray = data_structures::array::array(10);
    let getvector = data_structures::vector::vector(10);
    let gethashmap = data_structures::hashmap::hashmap();
    let heap = data_structures::binaryheap::binaryheap();
    let queue = data_structures::queue::queue();
    let enums = data_structures::enums::enums();
    let linked_list = data_structures::linkedlist::linkedlist();
    let binarytree = data_structures::binarytree::binarytree(10);

    // println!("Array: {:?}", getarray);
    // println!("Vector: {:?}", getvector);
    // println!("Hashmap: {:?}", gethashmap);
    // println!("Heap: {:?}", heap);
    // println!("Binary tree: {:?}", binarytree)
    // println!("Linked list: {:?}", linked_list);
    // println!("Enum: {:?}", enums);
    // println!("Queue: {:?}", queue);
}