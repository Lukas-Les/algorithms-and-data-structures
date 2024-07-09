use std::{arch::x86_64::_MM_FROUND_CUR_DIRECTION, fmt::Debug, io::Cursor, ops::Deref, thread::current};

use hashing::Hashable;

pub mod hashing {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    fn fnv1a_hash(bytes: &[u8]) -> u64 {
        let mut hash = FNV_OFFSET_BASIS;
        for &byte in bytes {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }


    pub trait Hashable {
        fn hash(&self) -> u64;
    }

    impl Hashable for &str {
        fn hash(&self) -> u64 {
            fnv1a_hash(self.as_bytes())
        }
    }

    impl Hashable for i32 {
        fn hash(&self) -> u64 {
            fnv1a_hash(&self.to_le_bytes())
        }
    }
}

const TABLE_SIZE: usize = 3;


struct Entry<K, V> {
    key: K,
    value: V,
    next: Option<Box<Entry<K, V>>>,
}

impl <K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            next: None,
        }
    }
}

struct HashTable<K, V: Debug> {
    table: [Option<Box<Entry<K, V>>>; TABLE_SIZE],
}

impl <K: Hashable + Eq, V: Debug> HashTable<K, V> {
    pub fn new() -> Self {
        Self {
            table: [(); TABLE_SIZE].map(|_| None),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> () {
        let index: usize = key.hash() as usize % TABLE_SIZE;
        let mut new_entry = Box::new(Entry::new(key,value));
        let current_entry = &mut self.table[index];
        match current_entry {
            Some(head) => {
                let mut cursor = head;
                loop {
                    if cursor.key == new_entry.key {
                        new_entry.next = cursor.next.take();
                        *cursor = new_entry;
                        return;
                    }
                    if cursor.next.is_none() {
                        cursor.next = Some(new_entry);
                        return;
                    }
                    cursor = cursor.next.as_mut().unwrap();
                }
            },
            None => *current_entry = Some(new_entry),
        }
    }

    pub fn get(&self, key: K) -> Option<&V>{
        let index: usize = key.hash() as usize % TABLE_SIZE;
        let mut current_entry = &self.table[index];
        while let Some(entry) = current_entry {
            if entry.key == key {
                return Some(&entry.value);
            }
            current_entry = &entry.next;
        }
        None
    }

    pub fn delete(&mut self, key: K) -> () {
        let index: usize = key.hash() as usize % TABLE_SIZE;
        let mut current_entry = &mut self.table[index];
        match current_entry {
            Some(head) => {
                if head.next.is_none() {
                    *current_entry = None;
                    return;
                }
                let mut previous = head;
                let mut cursor = &mut head.next;
                while let Some(ref mut entry) = cursor {
                    if entry.key == key {
                        previous.next = entry.next.take();
                    }
                }
                
            },
            None => {return;}
        }
    }

    pub fn print(&self) -> () {
        for (index, entry) in self.table.iter().enumerate() {
            print!("{}\t", index);
            let mut current_entry = entry;
            while let Some(entry) = current_entry {
                print!("{:?} ->", &entry.value);
                current_entry = &entry.next;
            }
            println!("None");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{hashing::Hashable, HashTable};

    #[test]
    fn test_fnv1a_hash_str() {
        let hash_sum = "hello".hash();
        assert_eq!(hash_sum, 11831194018420276491);
    }

    #[test]
    fn test_fnv1a_hash_i32() {
        let n: i32 = 69;
        let hash_sum = n.hash();
        assert_eq!(hash_sum, 3164654411128539488);
    }

    #[test]
    fn test_hash_table() {
        let mut dict = HashTable::<&str, &str>::new();
        dict.insert("greeting", "hello");
        dict.insert("farewell", "bye");
        dict.insert("movie", "horror");
        dict.insert("car", "fast");
        dict.insert("color", "green");
        dict.insert("shape", "square");
        dict.insert("pet", "dog");


        // let found = *dict.get("greeting").unwrap();
        // assert_eq!(found, "hello");

        // dict.delete("greeting");
        dict.print();
    }
}
