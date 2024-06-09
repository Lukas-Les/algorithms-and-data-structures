use rust::data_structures::trie::Trie;

#[test]
fn trie() {
    let mut trie = Trie::new();
    trie.insert("ab", "ab value", false).unwrap();
    trie.insert("ba", "ba value", false).unwrap();
    assert_eq!(trie.retrieve("ab").unwrap(), "ab value");
}
