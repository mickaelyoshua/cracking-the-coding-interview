// String Compression: Implement a method to perform basic string compression using the counts
// of repeated characters. For example, the string aabcccccaaa would become a2b1c5a3. If the
// "compressed" string would not become smaller than the original string, your method should return
// the original string. You can assume the string has only uppercase and lowercase letters (a - z).

pub fn compress(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(compress("abcdef"), "abcdef");
        assert_eq!(compress("aabb"), "aabb");
    }
}
