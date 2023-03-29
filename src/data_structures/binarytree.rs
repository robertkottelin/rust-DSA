#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, left: None, right: None }
    }

    fn insert(&mut self, value: T)
    where
        T: std::cmp::Ord,
    {
        if value <= self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn contains(&self, value: &T) -> bool
    where
        T: std::cmp::Ord,
    {
        if value == &self.value {
            true
        } else if value < &self.value {
            match &self.left {
                Some(left) => left.contains(value),
                None => false,
            }
        } else {
            match &self.right {
                Some(right) => right.contains(value),
                None => false,
            }
        }
    }
}

pub fn binarytree(input: i32) -> Node<i32>{
    let mut tree = Node::new(input);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);
    tree.insert(1);
    tree.insert(9);
    // println!("{:#?}", tree); // prints the entire tree structure
    // println!("Contains 1? {}", tree.contains(&1));
    // println!("Contains 6? {}", tree.contains(&6));
    tree
}