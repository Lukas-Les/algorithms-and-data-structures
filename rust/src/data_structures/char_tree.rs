use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    name: char,
    value: Option<String>,
    next: Link,
}

impl Node {
    pub fn new(name: char) -> Self {
        Self {
            name,
            value: None,
            next: None,
        }
    }
}

struct Tree {
    root: Link,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    fn append_recursive(&self, mut path: Vec<char>, mut previous: Link) {
        if path.is_empty() {
            return;
        }
        let first_char = path.remove(0);
        let new_node = Rc::new(RefCell::new(Node::new(first_char)));

        match previous {
            Some(node) => {
                let mut node_borrowed = node.borrow_mut();
                if node_borrowed.next.is_none() {
                    node_borrowed.next = Some(new_node.clone());
                }
                self.append_recursive(path, node_borrowed.next.clone());
            }
            None => {},
        }
    }

    pub fn append(&mut self, path: &str, value: String) {
        let mut path_vec: Vec<char> = path.chars().collect();

        // If the tree is empty, initialize the root
        if self.root.is_none() {
            if let Some(first_char) = path_vec.get(0) {
                let root_node = Rc::new(RefCell::new(Node::new(*first_char)));
                self.root = Some(root_node.clone());
                path_vec.remove(0); // Remove the first character as it's now the root
                self.append_recursive(path_vec, Some(root_node));
            }
        } else {
            self.append_recursive(path_vec, self.root.clone());
        }

        // Set the value of the last node
        let mut current = self.root.clone();
        for char in path.chars() {
            if let Some(current_node) = current.clone() {
                let mut current_borrowed = current_node.borrow_mut();
                if current_borrowed.name == char {
                    if current_borrowed.next.is_some() {
                        current = current_borrowed.next.clone();
                    } else {
                        current_borrowed.value = Some(value.clone());
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree() {
        let mut ch_tree = Tree::new();
        ch_tree.append("abc", "value1".to_string());

        // You can add assertions to verify the structure of the tree and the values
        if let Some(root) = ch_tree.root.clone() {
            let root_borrowed = root.borrow();
            assert_eq!(root_borrowed.name, 'a');
            assert!(root_borrowed.value.is_none());
            
            if let Some(next) = &root_borrowed.next {
                let next_borrowed = next.borrow();
                assert_eq!(next_borrowed.name, 'b');
                assert!(next_borrowed.value.is_none());
                
                if let Some(next_next) = &next_borrowed.next {
                    let next_next_borrowed = next_next.borrow();
                    assert_eq!(next_next_borrowed.name, 'c');
                    assert_eq!(next_next_borrowed.value.as_ref().unwrap(), "value1");
                }
            }
        }
    }
}
