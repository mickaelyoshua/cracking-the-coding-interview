// One Away: There are three types of edits that can be performed on strings: insert a character,
// remove a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.
//
// EXAMPLE
// pale, ple -> true
// pales, pale -> true
// pale, bale -> true
// pale, bake -> false
//
// BCR O(N)

use std::cmp::Ordering;

// Time: O(N) | Space: O(N)
pub fn is_one_away(s1: &str, s2: &str) -> bool {
    if s1.len().abs_diff(s2.len()) > 1 {
        return false;
    }

    let s1_vec: Vec<char> = s1.chars().collect();
    let s2_vec: Vec<char> = s2.chars().collect();

    let mut diff_count = 0;

    let (bigger, lesser) = match s1.len().cmp(&s2.len()) {
        Ordering::Equal => {
            // O(N)
            for i in 0..s1.len() {
                if s1_vec[i] != s2_vec[i] {
                    diff_count += 1;
                }
                if diff_count > 1 {
                    return false;
                }
            }
            return true;
        }
        Ordering::Greater => (s1_vec, s2_vec),
        Ordering::Less => (s2_vec, s1_vec),
    };

    let mut lesser_idx = 0;
    // O(N)
    for c in bigger {
        if lesser_idx < lesser.len() && c == lesser[lesser_idx] {
            lesser_idx += 1;
        } else {
            diff_count += 1;
            if diff_count > 1 {
                return false;
            }
        }
    }

    true
}

// Challenge: use only iterators to reduce space usage.
// Time: O(N) | Space: O(1)
pub fn is_one_away_o1(s1: &str, s2: &str) -> bool {
    if s1.len().abs_diff(s2.len()) > 1 {
        return false;
    }

    // SEU CÓDIGO AQUI: Use s1.chars() e s2.chars() ao invés de alocar Vecs.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&str, &str) -> bool) {
        assert!(f("pale", "ple"));
        assert!(f("pales", "pale"));
        assert!(f("pale", "bale"));
        assert!(!f("pale", "bake"));
        assert!(!f("abc", "bca"));
        assert!(!f("a", "abc"));
        assert!(!f("pales", "bale"));
    }

    #[test]
    fn test_one_away() {
        run_tests(is_one_away);
    }

    #[test]
    fn test_one_away_o1() {
        run_tests(is_one_away_o1);
    }
}
