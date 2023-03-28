pub fn data_structures() {
    // Array
    let mut array = [1, 2, 3, 4, 5];

    // Vector
    let mut vector = Vec::new();
    vector.push(1);

    // String
    let string = "Hello string";

    // Hash Map: A data structure that maps keys to values
    use std::collections::HashMap;
    let mut hashmap = HashMap::new();
    hashmap.insert("Alice", 25);

    // Binary Heaps: A priority queue data structure that orders elements based on their value
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(3);

    // LinkedLists: A sequence of elements that are connected by pointers
    use std::collections::LinkedList;
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);

    // Queues: A collection of elements that allows insertion at one end and removal from the other.
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);

    // Stacks: A collection of elements that allows insertion and removal from one end.
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Slices: A reference to a contiguous sequence of elements in a collection
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    // Tuples: A fixed-size sequence of elements of different types
    let tuple = (1, "hello", true);
    let (a, b, c) = tuple;
}
