// Given two strings, write a method to decide if one is a permutation of the other.
//
// Since it is mandatory to go through at least one of the strings completly, the
// BCR (Best Conceivable Runtime) is O(N)

use std::collections::HashMap;

pub fn sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

// O(2*N*logN)
pub fn check_permutation_sorting(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    if s1 == s2 {
        return true;
    }

    let sorted_s1 = sort(s1); // O(N*logN)
    let sorted_s2 = sort(s2); // O(N*logN)

    if sorted_s1 != sorted_s2 {
        return false;
    }

    true
}

// O(2N)
pub fn check_permutation_hashmap(s1: &str, s2: &str) -> bool {
    // Make sure strings have the same length
    if s1.len() != s2.len() {
        return false;
    }
    if s1 == s2 {
        return true;
    }

    let mut counts = HashMap::new();

    // O(N)
    for c in s1.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    // O(N)
    for c in s2.chars() {
        if let Some(count) = counts.get_mut(&c) {
            if *count == 0 {
                return false;
            }
            *count -= 1;
        } else {
            return false;
        }
    }

    // Not necessary, since s1.len == s2.len, so the early return of
    // *count == 0 is enough.
    // if counts.values().any(|count| *count != 0) {
    //     return false;
    // }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn run_tests_for(f: fn(&str, &str) -> bool) {
        // Happy path
        assert!(f("abc", "cba"), "Failed for valid permutation");
        assert!(
            f("listen", "silent"),
            "Failed for valid permutation 'listen'/'silent'"
        );

        // Base cases
        assert!(f("", ""), "Failed for empty strings");
        assert!(f("a", "a"), "Failed for identical characters");

        // Edge cases - Not permutations
        assert!(!f("abc", "ab"), "Failed for different lengths");
        assert!(
            !f("aab", "abb"),
            "Failed for different character counts"
        );
        assert!(!f("abc", "def"), "Failed for different characters");
        assert!(!f("God", "dog"), "Failed for case sensitivity");
        assert!(
            !f("god   ", "dog"),
            "Failed for significant whitespaces"
        );
    }

    #[test]
    fn test_check_permutation() {
        run_tests_for(check_permutation_sorting);
        run_tests_for(check_permutation_hashmap);
    }

    proptest! {
        #[test]
        fn test_implementations_agree(s1 in "\\PC*", s2 in "\\PC*") {
            let res_sorting = check_permutation_sorting(&s1, &s2);
            let res_hashmap = check_permutation_hashmap(&s1, &s2);
            prop_assert_eq!(res_sorting, res_hashmap);
        }
    }
}
