
struct Node {
    name: char,
    value: Option<String>,
    children: Vec<Box<Node>>,
}


impl Node {
    fn new(name: char) -> Self {
        Node {
            name: name,
            value: None,
            children: Vec::new(),
        }
    }

    fn get_child(&self, name: char) -> Option<&Box<Node>> {
        self.children.iter().find(|node| node.name == name)
    }

    fn get_child_mut(&mut self, name: char) -> Option<&mut Box<Node>> {
        self.children.iter_mut().find(|node| node.name == name)
    }
}


struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            root: None,
        }
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
        }
    }

    pub fn insert(&mut self, mut path: &str, value: &str) {
        if self.root.is_none() {
            let first_char = path.chars().next().unwrap();
            path = &path[1..];
            let new_node = Box::new(Node::new(first_char));
            self.root = Some(new_node);
        }
        
        let mut current_node = self.root.as_mut().unwrap();
        Self::insert_recursive(path, value, current_node);
    }

    fn get(&self, mut path: &str) -> Option<String> {
        if self.root.is_none() {
            return None;
        }
        let mut current_node = self.root.as_ref().unwrap();
        while !path.is_empty() {
            let first_char = path.chars().next().unwrap();
            path = &path[1..];
            match current_node.get_child(first_char) {
                Some(child) => current_node = child,
                None => break,
            }
        }
        current_node.value.clone()
    }
    
}

mod tests {
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
        tree.insert("abc", "ABC");
        assert_eq!(tree.get("abc").unwrap(), "ABC".to_string());
    }
}