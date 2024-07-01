use std::fmt::Debug;

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

const TABLE_SIZE: usize = 10;


struct HashTable<V: Debug> {
    table: [Option<V>; TABLE_SIZE],
}

impl <V: Debug> HashTable<V> {
    pub fn new() -> Self {
        Self {
            table: [(); TABLE_SIZE].map(|_| None),
        }
    }

    pub fn insert<K: Hashable>(&mut self, key: K, value: V) -> () 
    {
        let index: usize = key.hash() as usize % TABLE_SIZE;
        self.table[index] = Some(value);
    }

    pub fn print(&self) -> () {
        for i in 0..TABLE_SIZE {
            if let Some(value) = &self.table[i] {
                println!("{}: {:?}", i, value);
            }
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
        let mut dict: HashTable<&str> = HashTable::<&str>::new();
        dict.insert("greeting", "hello");
        dict.insert("farewell", "bye");
        dict.insert("color", "green");
        dict.insert("shape", "square");

        dict.print();
    }
}
