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

// Time: O(N) | Space: O(1)
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

// Time: O(N^2) | Space: O(1)
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

    fn run_tests_for(f: fn(&mut [char], usize)) {
        // Example
        let mut s: Vec<char> = "Mr John Smith    ".chars().collect();
        f(&mut s, 13);
        assert_eq!(s, "Mr%20John%20Smith".chars().collect::<Vec<char>>());

        // Empty string
        let mut s: Vec<char> = vec![];
        f(&mut s, 0);
        assert_eq!(s, vec![]);

        // No spaces
        let mut s: Vec<char> = "hello".chars().collect();
        f(&mut s, 5);
        assert_eq!(s, "hello".chars().collect::<Vec<char>>());

        // Only spaces
        let mut s: Vec<char> = "   ".chars().collect();
        f(&mut s, 1);
        assert_eq!(s, "%20".chars().collect::<Vec<char>>());

        // Starting spaces
        let mut s: Vec<char> = " a  ".chars().collect();
        f(&mut s, 2);
        assert_eq!(s, "%20a".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_urlify_brute_force() {
        run_tests_for(urlify_brute_force);
    }
    
    #[test]
    fn test_urlify_optimal() {
        run_tests_for(urlify);
    }
}
