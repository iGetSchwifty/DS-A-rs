pub mod node;

use node::*;

#[derive(Debug)]
pub struct LinkedList<T: Copy> {
    head: Option<Node<T>>,
    tail: Option<*mut Node<T>>
}

impl<T> LinkedList<T> where T: Copy {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    // O(1)
    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Node::new(value, None));
        } else {
            let old_head = self.head.take().unwrap();
            self.head = Some(Node::new(value, Some(old_head)));
        }
        if self.tail.is_none() {
            self.tail = Some(self.head.as_mut().unwrap());
        }
    }

    // O(1)
    pub fn append(&mut self, value: T) {
        let mut new_tail = Box::new(Node::new(value, None));
        let raw_tail: *mut _ = &mut *new_tail;

        if self.empty() {
            self.push(value);
        } else {
            unsafe { (*self.tail.unwrap()).next = Some(new_tail) };
        }
        self.tail = Some(raw_tail);
    }

    // O(index)
    pub fn node_for_index(&self, index: usize) -> Option<&Node<T>> {
        let mut current_index = 0;
        let mut current_node = self.head.as_ref();

        while current_index < index && current_node.is_some() {
            match current_node {
                Some(node) => {
                    if let Some(next_node) = &node.next {
                        current_node = Some(next_node);
                        current_index += 1;
                    } else {
                        current_node = None;
                    }
                },
                None => {
                    current_node = None;
                }
            }
        }
        current_node
    }

    // O(1)
    pub fn pop(&mut self) -> Option<Node<T>> {
        if self.empty() {
            None
        } else {
            let node = self.head.take();
            let mut next_node = node.unwrap();
            let optional_next = next_node.next.take();
            if optional_next.is_some() {
                self.head = Some(*optional_next.unwrap());
            } else {
                self.tail = None;
            }
            Some(next_node)
        }
    }
}

impl<T> Drop for LinkedList<T> where T: Copy {
    fn drop(&mut self) {
        let mut node = self.head.take();
        while let Some(mut next_node) = node.take() {
            let optional_next = next_node.next.take();
            if optional_next.is_some() {
                node = Some(*optional_next.unwrap());
            }
        }
    }
}