use std::collections::HashMap;

struct Node {
    children: HashMap<char, Node>,
    value: Option<String>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: HashMap::new(),
            value: None,
        }
    }
}


struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Node::new(),
        }
    }

    fn find_node_as_ref(&self, path: &str) -> Option<&Node> {
        let mut current_node = & self.root;
        for c in path.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                _ => return None,
            }
        }
        Some(current_node)

    }

    pub fn insert(&mut self, path: &str, value: &str, replace: bool) -> Result<(), String>{
        let mut current_node = &mut self.root;
        for c in path.chars() {
            current_node = current_node.children.entry(c).or_insert_with(|| Node::new());
        }
        if replace {
            current_node.value = Some(value.to_string());
            Ok(())
        } else {
            match &current_node.value {
                Some(exs_value) => return Err(format!("Cannot insert: path '{path}' is already occupied with '{exs_value}'")),
                None => {
                    current_node.value = Some(value.to_string());
                    Ok(())
                },
            }
        }
    }

    pub fn retrieve(&self, path: &str) -> Option<String> {
        match self.find_node_as_ref(path) {
            Some(node) => return node.value.clone(),
            _ => None,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie() {
        let trie = Trie::new();
        assert_eq!(trie.root.value, None);
    }

    #[test]
    fn test_insert_with_replace() {
        let mut trie = Trie::new();
        trie.insert("aa", "a value", false).unwrap();
        let first_child = trie.root.children.get(&'a').unwrap();
        let second_child = first_child.children.get(&'a').unwrap();
        assert_eq!(second_child.value.as_ref().unwrap(), "a value");
    }

    #[test]
    #[should_panic(expected = "Cannot insert: path 'ab' is already occupied with 'ab value'")]
    fn test_insert_no_replace() {
        let mut trie = Trie::new();
        trie.insert("ab", "ab value", true).unwrap();
        trie.insert("ab", "ab replace", false).unwrap();
        let first_child = trie.root.children.get(&'a').unwrap();
        let second_child = first_child.children.get(&'b').unwrap();
        assert_eq!(second_child.value.as_ref().unwrap(), "ab value");

    }

    #[test]
    fn test_retrieve() {
        let mut trie = Trie::new();
        trie.insert("ab", "ab value", false).unwrap();
        assert_eq!(trie.retrieve("ab").unwrap(), "ab value".to_string());
    }
}
