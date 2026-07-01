// Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and
// column are set to 0.

use std::collections::HashSet;

fn get_zero_rows_columns(matrix: &[Vec<i32>]) -> (Vec<usize>, Vec<usize>) {
    let mut rows = HashSet::new();
    let mut columns = HashSet::new();

    for (r, row) in matrix.iter().enumerate() {
        for (c, element) in row.iter().enumerate() {
            if *element == 0 {
                rows.insert(r);
                columns.insert(c);
            }
        }
    }
    (rows.into_iter().collect(), columns.into_iter().collect())
}

// Time: O(2 * M * N + M) | Space: O(M + N)
pub fn zero_matrix_with_extra_space(matrix: &mut [Vec<i32>]) {
    // Time: O(M * N) | Space: O(M + N)
    let (rows, columns) = get_zero_rows_columns(matrix);

    // O(M)
    for r in rows {
        matrix[r].fill(0);
    }

    // O(M * N)
    for row in matrix.iter_mut() {
        for &c in &columns {
            row[c] = 0;
        }
    }
}

// Time: O(M * N) | Space: O(1)
// Use first row and first column to map the 0s in the array
pub fn zero_matrix_in_place(matrix: &mut [Vec<i32>]) {
    let m = matrix.len();
    let n = matrix.first().map_or(0, |row| row.len());

    if m == 0 {
        return;
    }

    // O(M)
    let first_row_has_zero = matrix[0].contains(&0);

    // O(N)
    let first_column_has_zero = matrix.iter().any(|row| row.first() == Some(&0));

    // O(M * N)
    for r in 1..m {
        for c in 1..n {
            if matrix[r][c] == 0 {
                matrix[0][c] = 0;
                matrix[r][0] = 0;
            }
        }
    }

    // O(M)
    for row in matrix.iter_mut().skip(1) {
        if row[0] == 0 {
            row.fill(0);
        }
    }

    // O(M * N)
    for c in 1..n {
        if matrix[0][c] == 0 {
            matrix.iter_mut().for_each(|row| row[c] = 0);
        }
    }

    // O(M)
    if first_row_has_zero {
        matrix[0].fill(0);
    }

    // O(N)
    if first_column_has_zero {
        matrix.iter_mut().for_each(|row| row[0] = 0);
        // for row in matrix.iter_mut() {
        //     row[0] = 0;
        // }
    }
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
        let mut matrix = [
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];

        let expected = [
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        zero_matrix_in_place(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
