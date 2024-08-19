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

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Trie {
        Trie { root: Node::new() }
    }

    fn find_node_as_ref(&self, path: &str) -> Option<&Node> {
        let mut current_node = &self.root;
        for c in path.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                _ => return None,
            }
        }
        Some(current_node)
    }

    fn find_node_as_mut_ref(&mut self, path: &str) -> Option<&mut Node> {
        let mut current_node = &mut self.root;
        for c in path.chars() {
            match current_node.children.get_mut(&c) {
                Some(node) => current_node = node,
                _ => return None,
            }
        }
        Some(current_node)
    }

    pub fn insert(&mut self, path: &str, value: &str, replace: bool) -> Result<(), String> {
        let mut current_node = &mut self.root;
        for c in path.chars() {
            current_node = current_node
                .children
                .entry(c)
                .or_insert_with(|| Node::new());
        }
        if replace {
            current_node.value = Some(value.to_string());
            return Ok(());
        }
        match &current_node.value {
            Some(exs_value) => Err(format!(
                "Cannot insert: path '{path}' is already occupied with '{exs_value}'"
            )),
            None => {
                current_node.value = Some(value.to_string());
                Ok(())
            }
        }
    }

    pub fn retrieve(&self, path: &str) -> Option<String> {
        match self.find_node_as_ref(path) {
            Some(node) => return node.value.clone(),
            _ => None,
        }
    }

    fn recursive_remove(node: &mut Node, path: &[char]) -> Result<(), String> {
        if path.is_empty() {
            if node.value.is_some() {
                node.value = None;
                return Ok(());
            } else {
                return Err("Path doesn't exists".to_string());
            }
        }
        let c = path[0];
        if let Some(child) = node.children.get_mut(&c) {
            match Self::recursive_remove(child, &path[1..]) {
                Ok(_) => {
                    if child.children.is_empty() && child.value.is_none() {
                        node.children.remove(&c);
                    }
                    Ok(())
                }
                err => err,
            }
        } else {
            Err("Path doesn't exist".to_string())
        }
    }

    pub fn remove(&mut self, path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        let path_chars: Vec<char> = path.chars().collect();
        Self::recursive_remove(&mut self.root, &path_chars.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_trie() -> Trie {
        let mut trie = Trie::new();
        trie.insert("ab", "ab value", false).unwrap();
        trie.insert("ba", "ba value", false).unwrap();
        trie
    }

    #[test]
    fn empty_trie() {
        let trie = Trie::new();
        assert_eq!(trie.root.value, None);
    }

    #[test]
    fn insert_with_replace() {
        let mut trie = Trie::new();
        trie.insert("aa", "a value", false).unwrap();
        let first_child = trie.root.children.get(&'a').unwrap();
        let second_child = first_child.children.get(&'a').unwrap();
        assert_eq!(second_child.value.as_ref().unwrap(), "a value");
    }

    #[test]
    #[should_panic(expected = "Cannot insert: path 'ab' is already occupied with 'ab value'")]
    fn insert_no_replace() {
        let mut trie = Trie::new();
        trie.insert("ab", "ab value", true).unwrap();
        trie.insert("ab", "ab replace", false).unwrap();
        let first_child = trie.root.children.get(&'a').unwrap();
        let second_child = first_child.children.get(&'b').unwrap();
        assert_eq!(second_child.value.as_ref().unwrap(), "ab value");
    }

    #[test]
    fn retrieve() {
        let trie = setup_trie();

        assert_eq!(trie.retrieve("ab").unwrap(), "ab value".to_string());
    }

    #[test]
    fn remove() {
        let mut trie = setup_trie();
        assert_eq!(trie.retrieve("ab").unwrap(), "ab value".to_string());

        trie.remove("ab").unwrap();
        assert_eq!(trie.retrieve("ab"), None);
    }
}
