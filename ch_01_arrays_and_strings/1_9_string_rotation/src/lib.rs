// String Rotation: Assume you have a method isSubstring which checks if one word is a substring
// of another. Given two strings, s1 and s2, write code to check if s2 is a rotation of s1 using only one
// call to isSubstring (e.g., "waterbottle" is a rotation of "erbottlewat").

// Time: O(N) | Space: O(2N) where N is the length of s1
pub fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    if s1.is_empty() {
        return true;
    }

    // If s1 == xy then
    // s2 == yx
    // So, s1s1 == xyxy that contains yx (s2)
    let double_s1 = [s1, s1].concat();
    // O(N)
    double_s1.contains(s2)
}

// Time: O(N²) | Space: O(1)
// This challenge has no limitations to use isSubstring (contains)
pub fn is_rotation_space_o1(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    if s1.is_empty() {
        return true;
    }

    let n = s1.chars().count();

    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    // O(N)
    for i in 0..n {
        let mut match_found = true;

        // O(N)
        for (j, c2) in b2.iter().enumerate() {
            let rotated_idx = (i + j) % n;

            if b1[rotated_idx] != *c2 {
                match_found = false;
                break;
            }
        }

        if match_found {
            return true;
        }

        // Rust way
        // O(N)
        // let rotated_simulation = s1.chars().cycle().skip(i).take(n);
        // if rotated_simulation.eq(s2.chars()) {
        //     return true;
        // }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fns(f: fn(&str, &str) -> bool) {
        assert!(f("waterbottle", "erbottlewat"));
        assert!(f("hello", "llohe"));
        assert!(!f("hello", "olleh"));
        assert!(!f("hello", "lohe"));
        assert!(!f("waterbottle", "erbotlewatt"));
    }

    #[test]
    fn test_is_rotation() {
        test_fns(is_rotation);
        test_fns(is_rotation_space_o1);
    }
}
