// Implement an algorithm to determine if a string has all unique characters. What if you cannot use additional data structures?

// Since it is mandatory to go through the entire string to check if it has unique characters, the
// BCR (Best Conceivable Runtime) is O(N)
use std::collections::{HashMap, HashSet, hash_map::Entry};

pub fn is_all_unique_hash_map(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut map = HashMap::new();
    // O(N)
    for c in s.chars() {
        // Entry for in-place manipulation
        if let Entry::Vacant(e) = map.entry(c) {
            e.insert(1);
        } else {
            return false;
        }
    }
    true
}

// HashSet stores only keys without values.
// Based on sets. A collection of unique values.
pub fn is_all_unique_hash_set(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut set = HashSet::new();
    // O(N)
    for c in s.chars() {
        if !set.insert(c) {
            return false;
        }
    }

    true
}

// O(1) in capacity
// Bit Vector (only ASCII characters a-z)

pub mod bit_vector;

pub fn is_all_unique_no_ds(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut bv = bit_vector::BitVector::new();

    for c in s.chars() {
        let index = c as usize - 'a' as usize;
        if bv.check(index) {
            return false;
        }
        bv.set(index);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn run_tests_for(f: fn(&str) -> bool) {
        // Happy path
        assert!(!f("foo"), "Failed for 'foo'");
        assert!(f("bar"), "Failed for 'bar'");

        // Base cases
        assert!(f(""), "Failed for empty string");
        assert!(f("a"), "Failed for single character");
    }

    #[test]
    fn test_hash_map() {
        run_tests_for(is_all_unique_hash_map);
    }

    #[test]
    fn test_hash_set() {
        run_tests_for(is_all_unique_hash_set);
    }

    #[test]
    fn test_no_ds() {
        run_tests_for(is_all_unique_no_ds);
    }

    proptest! {
        #[test]
        fn test_all_implementations_agree(s in "[a-z]*") {
            let res_map = is_all_unique_hash_map(&s);
            let res_set = is_all_unique_hash_set(&s);
            let res_no_ds = is_all_unique_no_ds(&s);

            prop_assert_eq!(res_map, res_set);
            prop_assert_eq!(res_map, res_no_ds);
        }
    }
}
