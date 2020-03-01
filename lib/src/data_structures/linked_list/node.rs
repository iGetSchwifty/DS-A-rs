#[derive(Debug)]
pub struct Node<T: Copy> {
    pub value: T,
    pub next: Option<Box<Node<T>>>
}

impl<T: Copy> Node<T> {
    pub fn new(value: T, next: Option<Node<T>>) -> Self {
        if next.is_none() {
            Node {
                value: value,
                next: None
            }
        } else {
            Node {
                value: value,
                next: Some(Box::new(next.unwrap()))
            }
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