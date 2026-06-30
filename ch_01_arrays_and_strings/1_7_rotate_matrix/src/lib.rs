// Rotate Matrix: Given an image represented by an NxN matrix, where each pixel in the image is
// 4 bytes, write a method to rotate the image by 90 degrees. Can you do this in place?
//
// BCR O(N)

// Time: O(N²) | Space: O(1)
pub fn rotate_matrix(matrix: &mut [Vec<u32>]) {
    // Work on layers, concentric rings on the matrix.
    // Number of layer = n / 2. E.g.: 3 / 2 = 1
    //
    // Internal limit to change elements is first = layer / last = n - 1 - layer | first to last - 1
    // E.g.: line 0: [1, 2, 3]
    // first = 0 (element 1) (Fist iteration the layer is 0)
    // last = 3 - 1 - 0 = 2 (element 3) (will not include last element)

    let n = matrix.len();

    for layer in 0..(n / 2) {
        let first = layer;
        let last = n - 1 - layer;

        for i in first..last {
            // Calculate position from border
            let offset = i - first;

            let top = matrix[first][i];
            // Left to top
            matrix[first][i] = matrix[last - offset][first];
            // Inferior to left
            matrix[last - offset][first] = matrix[last][last - offset];
            // Right to inferior
            matrix[last][last - offset] = matrix[i][last];
            // Top to right
            matrix[i][last] = top;
        }
    }
}

// Time: O(N²) | Space: O(N²)
pub fn rotate_matrix_out_of_place(matrix: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let n = matrix.len();
    let mut rotated_matrix = vec![vec![0; n]; n];
    // Formula for rotation in 90 degrees clock wise
    // (new_i, new_j) = (j, n - 1 - i)

    // Rust way
    for (i, line) in matrix.iter().enumerate() {
        for (j, element) in line.iter().enumerate() {
            let (new_i, new_j) = (j, n - 1 - i);
            rotated_matrix[new_i][new_j] = *element;
        }
    }

    // for i in 0..n {
    //     for j in 0..n {
    //         let (new_i, new_j) = (j, n - 1 - i);
    //         rotated_matrix[new_i][new_j] = matrix[i][j];
    //     }
    // }

    rotated_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_rotation(mut matrix: Vec<Vec<u32>>, expected: Vec<Vec<u32>>) {
        assert_eq!(
            rotate_matrix_out_of_place(&matrix),
            expected,
            "out_of_place failed"
        );
        rotate_matrix(&mut matrix);
        assert_eq!(matrix, expected, "in_place failed");
    }

    #[test]
    fn test_rotate_matrix_empty() {
        let matrix: Vec<Vec<u32>> = vec![];
        let expected: Vec<Vec<u32>> = vec![];
        verify_rotation(matrix, expected);
    }

    #[test]
    fn test_rotate_matrix_1x1() {
        let matrix = vec![vec![1]];
        let expected = vec![vec![1]];
        verify_rotation(matrix, expected);
    }

    #[test]
    fn test_rotate_matrix_2x2() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![3, 1], vec![4, 2]];
        verify_rotation(matrix, expected);
    }

    #[test]
    fn test_rotate_matrix_3x3() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        verify_rotation(matrix, expected);
    }

    #[test]
    fn test_rotate_matrix_4x4() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];
        verify_rotation(matrix, expected);
    }
}
