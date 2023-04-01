use std::collections::VecDeque;

pub fn queue() -> VecDeque<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue
}


pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}


// let mut queue = Queue::new();

// queue.enqueue(1);
// queue.enqueue(2);
// queue.enqueue(3);

// assert_eq!(queue.len(), 3);

// assert_eq!(queue.dequeue(), Some(1));
// assert_eq!(queue.dequeue(), Some(2));
// assert_eq!(queue.dequeue(), Some(3));

// assert_eq!(queue.is_empty(), true);
