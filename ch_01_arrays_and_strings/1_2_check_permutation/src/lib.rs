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
    // Make sure have same length
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

    fn run_tests_for(f: fn(&str, &str) -> bool) {
        // Happy path
        assert!(f("abc", "cba"), "Falhou para permutação válida");
        assert!(
            f("listen", "silent"),
            "Falhou para permutação válida 'listen'/'silent'"
        );

        // Base cases
        assert!(f("", ""), "Falhou para strings vazias");
        assert!(f("a", "a"), "Falhou para caracteres idênticos");

        // Edge cases - Not permutations
        assert!(!f("abc", "ab"), "Falhou para comprimentos diferentes");
        assert!(
            !f("aab", "abb"),
            "Falhou para quantidade de caracteres diferentes"
        );
        assert!(!f("abc", "def"), "Falhou para caracteres diferentes");
        assert!(!f("God", "dog"), "Falhou para case sensitive");
        assert!(
            !f("god   ", "dog"),
            "Falhou para espaços em branco significativos"
        );
    }

    #[test]
    fn test_check_permutation() {
        run_tests_for(check_permutation_sorting);
        run_tests_for(check_permutation_hashmap);
    }
}
