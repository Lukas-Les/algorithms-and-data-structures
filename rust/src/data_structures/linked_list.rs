use std::clone;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: Option<T>,
    next: Link<T>
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
    head: Link<T>
}

impl <T: Debug> LinkedList <T>{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(Some(value))));
        if self.head.is_none() {
            self.head = Some(new_node);
        }
        let mut cursor = &self.head.unwrap();
        // match &self.head {
        //     Some(head) => {
        //         let mut cursor = &Rc::clone(head);
        //         while let Some(next) = cursor.borrow_mut().next {
        //             cursor = &next;
        //         }
        //     },
        //     None => {self.head = Some(new_node)}
        // }
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
