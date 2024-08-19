#[derive(Debug)]
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
        self.children.iter().find(|&node| node.name == name)
    }

    fn get_child_mut(&mut self, name: char) -> Option<&mut Box<Node>> {
        self.children.iter_mut().find(|node| node.name == name)
    }
}

struct Tree {
    root: Vec<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
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
            println!("pushing new node: {}", &first_char);
            current_node.children.push(Box::new(Node::new(first_char)));
            Self::insert_recursive(path, value, current_node.children.last_mut().unwrap())
        }
    }

    pub fn insert(&mut self, mut path: &str, value: &str) {
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        if self.root.is_empty() {
            println!("pushing new node: {}", &first_char);
            let new_node = Box::new(Node::new(first_char));
            self.root.push(new_node);
        }

        if let Some(current_node) = self.root.iter_mut().find(|n| n.name == first_char) {
            Self::insert_recursive(path, value, current_node);
        }
    }

    fn get(&self, mut path: &str) -> Option<String> {
        if self.root.is_empty() {
            return None;
        }
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        let mut current_node = self.root.iter().find(|&n| n.name == first_char)?;
        while !path.is_empty() {
            println!("-----");
            println!("Node: {}", &current_node.name);
            print!("Children: ");
            current_node.children.iter().for_each(|n| print!("{} ", &n.name));
            print!("\n");
            println!("first char: {}", &first_char);
            println!("-----");
            path = &path[1..];
            if let Some(child) = current_node.get_child(first_char) {
                current_node = child;
            };
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
        // tree.insert("aab", "AAB");
        // tree.insert("bac", "BAC");
        assert_eq!(tree.get("abc").unwrap(), "ABC".to_string());
        // assert_eq!(tree.get("aab").unwrap(), "AAB".to_string());
        // assert_eq!(tree.get("bac").unwrap(), "BAC".to_string());
    }
}
