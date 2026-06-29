// String Compression: Implement a method to perform basic string compression using the counts
// of repeated characters. For example, the string aabcccccaaa would become a2b1c5a3. If the
// "compressed" string would not become smaller than the original string, your method should return
// the original string. You can assume the string has only uppercase and lowercase letters (a - z).
//
// BCR O(N)
// Mandatory Space is O(N) (return a new String)

use std::fmt::Write;

// Time: O(N) | Space: O(2N)
pub fn compress_vec(s: &str) -> String {
    // Space: O(N)
    let mut compressed = String::new();
    // Space: O(N)
    let s_iter: Vec<char> = s.chars().collect();
    if s_iter.len() <= 2 {
        return s.into();
    }

    let mut reference_idx: usize = 0;
    let mut char_count = 1;
    // Time: O(N)
    for read_idx in 1..s_iter.len() {
        if s_iter[read_idx] == s_iter[reference_idx] {
            char_count += 1;
        } else {
            let _ = write!(&mut compressed, "{}{}", s_iter[reference_idx], char_count);
            char_count = 1;
            reference_idx = read_idx;
        }
        if compressed.len() >= s.len() {
            return s.into();
        }
    }
    let _ = write!(&mut compressed, "{}{}", s_iter[reference_idx], char_count);
    if compressed.len() >= s.len() {
        return s.into();
    }

    compressed
}

// Time: O(N) | Space: O(N)
pub fn compress_iter(s: &str) -> String {
    if s.len() <= 2 {
        return s.into();
    }
    // Space: O(N)
    let mut compressed = String::new();
    let mut chars = s.chars();

    let Some(mut last_char) = chars.next() else {
        return s.into();
    };

    let mut count = 1;
    for c in chars {
        if c == last_char {
            count += 1;
        } else {
            let _ = write!(&mut compressed, "{last_char}{count}");
            count = 1;
            last_char = c;
        }
        if compressed.len() >= s.len() {
            return s.into();
        }
    }
    let _ = write!(&mut compressed, "{last_char}{count}");
    if compressed.len() >= s.len() {
        return s.into();
    }

    // Time: O(N)
    compressed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&str) -> String) {
        // Base cases provided
        assert_eq!(f("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(f("abcdef"), "abcdef");
        assert_eq!(f("aabb"), "aabb");

        // Edge cases
        assert_eq!(f(""), ""); // Empty string
        assert_eq!(f("a"), "a"); // Single character
        assert_eq!(f("aa"), "aa"); // Two characters (length is 2, compressed would be 2, so return original)
        assert_eq!(f("aaa"), "a3"); // Three characters

        // Early abort mid-string / length boundary cases
        assert_eq!(f("ababab"), "ababab"); // High entropy
        assert_eq!(f("aabbccddee"), "aabbccddee"); // Always length 2 for each, compressed length == original length

        // Case sensitivity (though problem says a-z/A-Z, they are treated as distinct)
        assert_eq!(f("aA"), "aA");
        assert_eq!(f("aaAA"), "aaAA");
        assert_eq!(f("aaaaaAAAAA"), "a5A5");
    }

    #[test]
    fn test_compress() {
        run_tests(compress_vec);
        run_tests(compress_iter);
    }
}
