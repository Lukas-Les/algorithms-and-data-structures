//! This module provides a Char Tree - structure, that lets you to store and retrieve a given value to a given path.
//! Example:
//! ```
//! use rust::data_structures::char_tree::Tree;
//! 
//! let mut tree = Tree::new();
//! tree.insert("mypath", "somevalue");
//! let result = tree.get("mypath").unwrap();
//! assert_eq!(result, "somevalue");
//! tree.delete("mypath");
//! assert_eq!(tree.get("mypath"), None);
//! ```



#[derive(Debug)]
struct Node {
    name: char,
    value: Option<String>,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(name: char) -> Self {
        println!("creating new node: {}", &name);
        Node {
            name: name,
            value: None,
            children: Vec::new(),
        }
    }

    fn get_child_ref(&self, name: char) -> Option<&Box<Node>> {
        self.children.iter().find(|node| node.name == name)
    }

    fn get_child_mut(&mut self, name: char) -> Option<&mut Box<Node>> {
        self.children.iter_mut().find(|node| node.name == name)
    }
}


/// The Tree struct allows you to store &str values on a provided char path;
/// Use insert(path: &str, value: &str) to insert value and
/// get(path: &str) to retireve it.
pub struct Tree {
    root: Vec<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: Vec::new() }
    }

    fn insert_recursive(mut path: &str, value: &str, mut current_node: &mut Box<Node>) {
        if path.is_empty() {
            current_node.value = Some(value.to_string());
            return;
        }
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        if let Some(child) = current_node.get_child_mut(first_char) {
            Self::insert_recursive(path, value, child)
        } else {
            current_node.children.push(Box::new(Node::new(first_char)));
            Self::insert_recursive(path, value, current_node.children.last_mut().unwrap())
        }
    }

    /// Inserts given valia to a given path.
    pub fn insert(&mut self, mut path: &str, value: &str) {
        if path.is_empty() {
            return;
        }

        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        if self.root.is_empty() {
            let new_node = Box::new(Node::new(first_char));
            self.root.push(new_node);
            Self::insert_recursive(path, value, self.root.iter_mut().last().unwrap());
            return;
        }

        if let Some(current_node) = self.root.iter_mut().find(|n| n.name == first_char) {
            Self::insert_recursive(path, value, current_node);
        } else {
            let new_node = Box::new(Node::new(first_char));
            self.root.push(new_node);
            Self::insert_recursive(path, value, self.root.iter_mut().last().unwrap());
        }
    }

    /// This method gets a value from a given path.
    pub fn get(&self, mut path: &str) -> Option<String> {
        if self.root.is_empty() {
            return None;
        }
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        let mut current_node = self.root.iter().find(|&n| n.name == first_char)?;
        while !path.is_empty() {
            let first_char = path.chars().next().unwrap();
            path = &path[1..];
            if let Some(child) = current_node.get_child_ref(first_char) {
                current_node = child;
            };
        }
        current_node.value.clone()
    }

    /// While shallow deleting value from a targeted path, this methot counts childless nodes,
    /// and after that it can easily deep delete empty nodes;
    pub fn delete(&mut self, mut path: &str) {
        if self.root.is_empty() || path.is_empty() {
            return;
        }
        let path_clone = path.to_string();
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        let mut current_node = match self.root.iter_mut().find(|n| n.name == first_char){
            Some(node) => node,
            None => {return;},
        };
        let mut no_other_childs_count: usize = 0;
        while !path.is_empty() {
            if current_node.children.len() == 1 {
                no_other_childs_count += 1;
            }
            let first_char = path.chars().next().unwrap();
            path = &path[1..];
            current_node = match current_node.get_child_mut(first_char){
                Some(node) => node,
                None => {return;},
            };
        }
        current_node.value = None;

        // TODO:  deep deletion
        // let mut path = &path_clone[0..=no_other_childs_count];
        // let first_char = path.chars().next().unwrap();
        // path = &path[1..];
        // current_node = self.root.iter_mut().find(|n| n.name == first_char).unwrap();

        // for c in path.chars() {
        //     current_node = current_node.get_child_mut(c).unwrap();
        // }

        // let mut prev = current_node;
        // let mut current_node = prev.children.last_mut().unwrap();
        // let mut next = current_node.children.last().unwrap();


    }
}

mod tests {
    use crate::data_structures::trie;

    use super::*;

    #[test]
    fn test_node() {
        let mut node = Node::new('a');
        node.children.push(Box::new(Node::new('b')));
        assert_eq!(node.get_child_mut('b').unwrap().name, 'b');
    }

    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        tree.insert("", "A");

        tree.insert("a", "A");
        tree.insert("ab", "AB");
        tree.insert("abc", "ABC");
        tree.insert("edc", "EDC");
        assert_eq!(tree.get("a").unwrap(), "A".to_string());
        assert_eq!(tree.get("ab").unwrap(), "AB".to_string());
        assert_eq!(tree.get("abc").unwrap(), "ABC".to_string());
        assert_eq!(tree.get("edc").unwrap(), "EDC".to_string());

        tree.delete("abc");
        assert_eq!(tree.get("abc"), None);
    }
}
