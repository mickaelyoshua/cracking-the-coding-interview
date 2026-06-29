// Rotate Matrix: Given an image represented by an NxN matrix, where each pixel in the image is
// 4 bytes, write a method to rotate the image by 90 degrees. Can you do this in place?

pub fn rotate_matrix(matrix: &mut Vec<Vec<u32>>) {
    todo!()
}

pub fn rotate_matrix_out_of_place(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        
        let expected = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3]
        ];
        
        let out_of_place = rotate_matrix_out_of_place(&matrix);
        assert_eq!(out_of_place, expected);

        rotate_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
