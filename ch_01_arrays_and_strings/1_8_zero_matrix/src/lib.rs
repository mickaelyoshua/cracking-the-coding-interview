// Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and
// column are set to 0.

// Time: O(M * N) | Space: O(M + N)
pub fn zero_matrix_with_extra_space(matrix: &mut [Vec<i32>]) {
    unimplemented!()
}

// Time: O(M * N) | Space: O(1)
pub fn zero_matrix_in_place(matrix: &mut [Vec<i32>]) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix_with_extra_space() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];

        let expected = vec![
            vec![1, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![9, 0, 11, 0],
            vec![0, 0, 0, 0],
        ];

        zero_matrix_with_extra_space(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_zero_matrix_in_place() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];

        let expected = vec![
            vec![1, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![9, 0, 11, 0],
            vec![0, 0, 0, 0],
        ];

        // zero_matrix_in_place(&mut matrix);
        // assert_eq!(matrix, expected);
    }
}
