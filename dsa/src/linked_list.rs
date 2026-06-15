type Link = Option<Box<Node<T>>>

struct Node<T> {
    val: T,
    next: Link,
}

pub struct LinkedList<T> {
    pub head: Link,
}

impl LinkedList<T> {
    pub fn new(val: T) -> Self {
        uninplemented!()
    }
}