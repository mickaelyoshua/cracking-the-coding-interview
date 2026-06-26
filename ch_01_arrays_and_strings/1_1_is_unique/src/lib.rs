// Implement an algorithm to determine if a string has all unique characters. What if you cannot use additional data structures?

// Since it is mandatory to go through the entire string to check if it has unique characters, the
// BCR (Best Conceivable Runtime) is O(N)
use std::collections::{HashMap, HashSet, hash_map::Entry};

// Time: O(N) | Space: O(N)
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
// Time: O(N) | Space: O(N)
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

// Bit Vector (only ASCII characters a-z)
// Time: O(N) | Space: O(1)

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

    fn run_tests_for(f: fn(&str) -> bool) {
        // Happy path - Unique
        assert!(f("bar"), "Failed for 'bar'");
        assert!(f("abcdefg"), "Failed for 'abcdefg'");

        // Happy path - Not Unique
        assert!(!f("foo"), "Failed for 'foo'");
        assert!(!f("hello"), "Failed for 'hello'");

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
}
