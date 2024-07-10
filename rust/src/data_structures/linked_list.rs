use std::{fmt::Debug, thread::current};


struct Node<T> {
    value: Option<T>,
    next: Option<Box<Node<T>>>
}

impl <T: Debug> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}


struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl <T: Debug> LinkedList <T>{
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn insert(&mut self, item: T) {
        let new_node = Some(Box::new(Node::new(Some(item))));
        let mut current = &mut self.head;

        while let Some(ref mut boxed_node) = &current {
            if boxed_node.next.is_none() {
                boxed_node.next = new_node;
                return;
            }
            current = &mut boxed_node.next;
        }
        *current = new_node;
    }
}


#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_linked_list() {
        let new_list = LinkedList::<&str>::new();
    }
}