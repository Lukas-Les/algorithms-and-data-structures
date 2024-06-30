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


#[cfg(test)]
mod tests {
    use super::hashing::Hashable;

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
}
