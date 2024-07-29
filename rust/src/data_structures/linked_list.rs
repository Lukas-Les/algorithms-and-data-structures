use std::fmt::Debug;

struct Node<T> {
    value: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl <T: Debug> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}


struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl <T: Debug> SinglyLinkedList <T>{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node{ value: Some(value), next: None });
        match self.head {
            Some(ref mut node) => {
                let mut current = node.as_mut();
                while let Some(ref mut next) = current.next {
                    current = next.as_mut();
                }
                current.next = Some(new_node);
            },
            None => {self.head = Some(new_node);},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn test_linked_list() {
        let mut list = SinglyLinkedList::<&str>::new();
        list.append("first");
        list.append("second");
        assert_eq!(list.head.as_ref().unwrap().value, Some("first"));
        assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().value, Some("second"));
    }
}