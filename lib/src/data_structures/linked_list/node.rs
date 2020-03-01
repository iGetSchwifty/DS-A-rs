#[derive(Debug)]
pub struct Node<T: Copy> {
    pub value: T,
    pub next: Box<Option<Node<T>>>
}

impl<T: Copy> Node<T> {
    pub fn new(value: T, next: Option<Node<T>>) -> Self {
        Node {
            value: value,
            next: Box::new(next)
        }
    }
}

impl<T: Copy> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node {
            value: self.value,
            next: self.next.clone()
        }
    }
}