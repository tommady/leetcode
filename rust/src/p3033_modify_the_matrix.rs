// Given a 0-indexed m x n integer matrix matrix, create a new 0-indexed matrix called answer. Make answer equal to matrix, then replace each element with the value -1 with the maximum element in its respective column. Return the matrix answer. Example 1: Input: matrix = [[1,2,-1],[4,-1,6],[7,8,9]] Output: [[1,2,9],[4,8,6],[7,8,9]] Explanation: The diagram above shows the elements that are changed (in blue). - We replace the value in the cell [1][1] with the maximum value in the column 1, that is 8. - We replace the value in the cell [0][2] with the maximum value in the column 2, that is 9. Example 2: Input: matrix = [[3,-1],[5,2]] Output: [[3,2],[5,2]] Explanation: The diagram above shows the elements that are changed (in blue). Constraints: m == matrix.length n == matrix[i].length 2 = m, n = 50 -1 = matrix[i][j] = 100 The input is generated such that each column contains at least one non-negative integer.
//
//
// [[1,2,-1],[4,-1,6],[7,8,9]]
// [[3,-1],[5,2]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut replaces = Vec::with_capacity(matrix.len() * matrix[0].len());
        let mut col_max_m = vec![0; matrix[0].len()];
        let mut matrix = matrix;

        for (col, _item) in matrix[0].iter().enumerate() {
            for (raw, _item) in matrix.iter().enumerate() {
                if matrix[raw][col] == -1 {
                    replaces.push((raw, col));
                }
                if matrix[raw][col] > col_max_m[col] {
                    col_max_m[col] = matrix[raw][col];
                }
            }
        }

        for &(raw, col) in replaces.iter() {
            matrix[raw][col] = col_max_m[col];
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3033_solution() {
        // 1, 2,-1
        // 4,-1, 6
        // 7, 8, 9
        assert_eq!(
            vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]],
            Solution::modified_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![vec![3, 2], vec![5, 2]],
            Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]])
        );
    }
}
