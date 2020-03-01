pub mod node;

use node::*;
// Turns out LL in rust is harder than it needs to be. Skip and come back to so I can keep reading...
#[derive(Debug)]
pub struct LinkedList<T: Copy> {
    head: Option<Node<T>>,
    tail: Option<Node<T>> // Consider making this a ref and having ownership be from the node...
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Node::new(value, self.head.clone()));
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }

    pub fn append(&mut self, value: T) {
        if self.empty() {
            self.push(value);
        } else {
            if let Some(tail) = &mut self.tail {
                tail.next = Box::new(Some(Node::new(value, None)));
                self.tail = *tail.next.clone();
            }
        }
    }
}