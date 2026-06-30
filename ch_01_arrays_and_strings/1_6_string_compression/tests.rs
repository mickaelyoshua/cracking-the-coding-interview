fn compress_vec(s: &str) -> String {
    let mut compressed = String::new();
    let s_iter: Vec<char> = s.chars().collect();
    if s_iter.is_empty() { return s.into(); } // adding this since 1..0 panic otherwise? No 1..0 is empty
    let mut reference_idx: usize = 0;
    let mut char_count = 1;
    for read_idx in 1..s_iter.len() {
        if s_iter[read_idx] == s_iter[reference_idx] {
            char_count += 1;
        } else {
            use std::fmt::Write;
            let _ = write!(&mut compressed, "{}{}", s_iter[reference_idx], char_count);
            char_count = 1;
            reference_idx = read_idx;
        }
        if read_idx == s_iter.len() - 1 {
            use std::fmt::Write;
            let _ = write!(&mut compressed, "{}{}", s_iter[reference_idx], char_count);
        }
        if compressed.len() >= s_iter.len() {
            return s.into();
        }
    }
    compressed
}

fn main() {
    println!("{:?}", compress_vec("aabcccccaaa"));
    println!("{:?}", compress_vec("abcdef"));
    println!("{:?}", compress_vec("aabb"));
    println!("{:?}", compress_vec("a"));
}
