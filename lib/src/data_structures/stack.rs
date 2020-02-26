pub struct Stack<T> {
    storage: Vec<T>
}

impl<T> Stack<T> {
    pub fn new(data: Vec<T>) -> Self {
        Stack {
            storage: data
        }
    }

    pub fn push(&mut self, element: T) {
        self.storage.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.storage.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.storage.last()
    }

    pub fn empty(&self) -> bool {
        self.peek().is_none()
    }
}