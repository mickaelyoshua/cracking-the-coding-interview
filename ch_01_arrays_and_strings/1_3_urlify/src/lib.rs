// Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the "true"
// length of the string. (Note: If implementing in Java, please use a character array so that you can
// perform this operation in place.)
//
// EXAMPLE
// Input:  "Mr John Smith    ", 13
// Output: "Mr%20John%20Smith"

pub fn urlify(s: &mut [char], true_length: usize) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_urlify_example() {
        let mut s: Vec<char> = "Mr John Smith    ".chars().collect();
        urlify(&mut s, 13);
        let expected: Vec<char> = "Mr%20John%20Smith".chars().collect();
        assert_eq!(s, expected);
    }

    #[test]
    fn test_urlify_empty() {
        let mut s: Vec<char> = vec![];
        urlify(&mut s, 0);
        let expected: Vec<char> = vec![];
        assert_eq!(s, expected);
    }

    proptest! {
        #[test]
        fn test_urlify_proptest(s in "[a-zA-Z0-9 ]*") {
            let s = s.trim_end(); // Remove trailing spaces to simplify true_length
            let true_length = s.chars().count();
            let spaces = s.chars().filter(|&c| c == ' ').count();

            let mut buf: Vec<char> = s.chars().collect();
            // Add extra space needed for substitutions (' ' -> '%20' requires 2 extra spaces per whitespace)
            buf.extend(std::iter::repeat_n(' ', spaces * 2));

            let mut expected = String::new();
            for c in s.chars() {
                if c == ' ' {
                    expected.push_str("%20");
                } else {
                    expected.push(c);
                }
            }
            let expected_chars: Vec<char> = expected.chars().collect();

            urlify(&mut buf, true_length);
            prop_assert_eq!(buf, expected_chars);
        }
    }
}
