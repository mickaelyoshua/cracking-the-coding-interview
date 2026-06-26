// Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palindrome.
// A palindrome is a word or phrase that is the same forwards and backwards. A permutation
// is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
// You can ignore casing and non-letter characters.
//
// EXAMPLE
// Input: Tact Coa
// Output: True (permutations: "taco cat", "atco cta", etc.)
//
// Since it is mandatory to go through the entire string to check, the BCR is O(N)

use std::collections::HashMap;

// O(2N)
pub fn is_palindrome_permutation_hash(s: &str) -> bool {
    let clean_s = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase());

    let mut char_counts = HashMap::new();

    // O(N)
    for c in clean_s {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    let mut count_odds = 0;

    // O(N)
    for count in char_counts.values() {
        if count % 2 != 0 {
            count_odds += 1;
        }
        if count_odds > 1 {
            return false;
        }
    }
    true
}

pub mod bit_vector;
use bit_vector::BitVector;

// O(N)
pub fn is_palindrome_permutation_bit_vector(s: &str) -> bool {
    let clean_s = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase());

    let mut bv = BitVector::new();

    // O(N)
    for c in clean_s {
        let index = c as usize - 'a' as usize;
        bv.toggle(index);
    }

    // Rust way of doing it
    // bv.data.count_ones() <= 1
    (bv.data & (bv.data.wrapping_sub(1))) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn run_happy_path_true(f: fn(&str) -> bool) {
        // Ignores case and spaces
        assert!(f("Tact Coa")); // taco cat
        assert!(f("aabbc")); // abcba
        assert!(f("aab")); // aba
    }

    fn run_happy_path_false(f: fn(&str) -> bool) {
        assert!(!f("Tact Coaa")); // two odd chars: t, o
        assert!(!f("aabbcd")); // odd chars: c, d
        assert!(!f("ab")); // odd chars: a, b
    }

    fn run_edge_cases(f: fn(&str) -> bool) {
        assert!(f("")); // empty string
        assert!(f("a")); // single char
        assert!(f("  ")); // spaces only
        assert!(f("A man a plan a canal Panama")); // famous long palindrome
    }

    #[test]
    fn test_hashmap_happy_path_true() {
        run_happy_path_true(is_palindrome_permutation_hash);
    }
    #[test]
    fn test_bit_vector_happy_path_true() {
        run_happy_path_true(is_palindrome_permutation_bit_vector);
    }

    #[test]
    fn test_hashmap_happy_path_false() {
        run_happy_path_false(is_palindrome_permutation_hash);
    }
    #[test]
    fn test_bit_vector_happy_path_false() {
        run_happy_path_false(is_palindrome_permutation_bit_vector);
    }

    #[test]
    fn test_hashmap_edge_cases() {
        run_edge_cases(is_palindrome_permutation_hash);
    }
    #[test]
    fn test_bit_vector_edge_cases() {
        run_edge_cases(is_palindrome_permutation_bit_vector);
    }

    proptest! {
        #[test]
        fn test_hashmap_property(s in "[a-zA-Z ]*") {
            run_property(s, is_palindrome_permutation_hash)?;
        }
        #[test]
        fn test_bit_vector_property(s in "[a-zA-Z ]*") {
            run_property(s, is_palindrome_permutation_bit_vector)?;
        }
    }

    fn run_property(
        s: String,
        f: fn(&str) -> bool,
    ) -> Result<(), proptest::test_runner::TestCaseError> {
        let mut counts = std::collections::HashMap::new();
        for c in s
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            *counts.entry(c).or_insert(0) += 1;
        }

        let odd_counts = counts.values().filter(|&&v| v % 2 != 0).count();
        let expected = odd_counts <= 1;

        prop_assert_eq!(f(&s), expected);
        Ok(())
    }
}
