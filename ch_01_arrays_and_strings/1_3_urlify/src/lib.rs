// Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the "true"
// length of the string. (Note: If implementing in Java, please use a character array so that you can
// perform this operation in place.)
//
// EXAMPLE
// Input:  "Mr John Smith    ", 13
// Output: "Mr%20John%20Smith"
//
// Since it is mandatory to go through the string entirely, the
// Best Conceivable Runtime (BCR) is O(N)

// O(2N)
pub fn urlify(s: &mut [char], true_length: usize) {
    let mut space_count = 0;
    // O()
    for c in s.iter().take(true_length) {
        if *c == ' ' {
            space_count += 1;
        }
    }

    if space_count == 0 {
        return;
    }

    // final length - 1
    let mut write_index = true_length + space_count * 2 - 1;

    // O(N)
    for read_index in (0..true_length).rev() {
        if s[read_index] == ' ' {
            s[write_index] = '0';
            s[write_index - 1] = '2';
            s[write_index - 2] = '%';
            // .saturating_sub will subtract without underflow
            // write_index is an usize and can not go under 0
            write_index = write_index.saturating_sub(3);
        } else {
            s[write_index] = s[read_index];
            write_index = write_index.saturating_sub(1);
        }
    }
}

// O(N²)
pub fn urlify_brute_force(s: &mut [char], true_length: usize) {
    let mut current_len = true_length;
    // O(N)
    for i in (0..true_length).rev() {
        if s[i] == ' ' {
            // O(N)
            for j in (i + 1..current_len).rev() {
                s[j + 2] = s[j];
            }
            s[i] = '%';
            s[i + 1] = '2';
            s[i + 2] = '0';
            current_len += 2;
        }
    }
    println!("{s:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn run_urlify_example(f: fn(&mut [char], usize)) {
        let mut s: Vec<char> = "Mr John Smith    ".chars().collect();
        f(&mut s, 13);
        let expected: Vec<char> = "Mr%20John%20Smith".chars().collect();
        assert_eq!(s, expected);
    }

    fn run_urlify_empty(f: fn(&mut [char], usize)) {
        let mut s: Vec<char> = vec![];
        f(&mut s, 0);
        let expected: Vec<char> = vec![];
        assert_eq!(s, expected);
    }

    #[test]
    fn test_urlify_brute_force_example() {
        run_urlify_example(urlify_brute_force);
    }
    #[test]
    fn test_urlify_optimal_example() {
        run_urlify_example(urlify);
    }

    #[test]
    fn test_urlify_brute_force_empty() {
        run_urlify_empty(urlify_brute_force);
    }
    #[test]
    fn test_urlify_optimal_empty() {
        run_urlify_empty(urlify);
    }

    proptest! {
        #[test]
        fn test_urlify_brute_force_proptest(s in "[a-zA-Z0-9 ]*") {
            run_proptest(s, urlify_brute_force)?;
        }

        #[test]
        fn test_urlify_optimal_proptest(s in "[a-zA-Z0-9 ]*") {
            run_proptest(s, urlify)?;
        }
    }

    fn run_proptest(
        s: String,
        f: fn(&mut [char], usize),
    ) -> Result<(), proptest::test_runner::TestCaseError> {
        let s = s.trim_end();
        let true_length = s.chars().count();
        let spaces = s.chars().filter(|&c| c == ' ').count();

        let mut buf: Vec<char> = s.chars().collect();
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

        f(&mut buf, true_length);
        prop_assert_eq!(buf, expected_chars);
        Ok(())
    }
}
