const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;


fn fnv1a_hash(s: &str) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in s.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv1a_hash() {
        let some_str = "hello";
        let hash_sum = fnv1a_hash(some_str);
        assert_eq!(hash_sum, 11831194018420276491);
    }
}
