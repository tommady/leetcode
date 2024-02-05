// You are given a 0-indexed m x n integer matrix mat and an integer k. You have to cyclically right shift odd indexed rows k times and cyclically left shift even indexed rows k times. Return true if the initial and final matrix are exactly the same and false otherwise. Example 1: Input: mat = [[1,2,1,2],[5,5,5,5],[6,3,6,3]], k = 2 Output: true Explanation: Initially, the matrix looks like the first figure. Second figure represents the state of the matrix after one right and left cyclic shifts to even and odd indexed rows. Third figure is the final state of the matrix after two cyclic shifts which is similar to the initial matrix. Therefore, return true. Example 2: Input: mat = [[2,2],[2,2]], k = 3 Output: true Explanation: As all the values are equal in the matrix, even after performing cyclic shifts the matrix will remain the same. Therefeore, we return true. Example 3: Input: mat = [[1,2]], k = 1 Output: false Explanation: After one cyclic shift, mat = [[2,1]] which is not equal to the initial matrix. Therefore we return false. Constraints: 1 = mat.length = 25 1 = mat[i].length = 25 1 = mat[i][j] = 25 1 = k = 50
//
//
// [[1,2,1,2],[5,5,5,5],[6,3,6,3]]
// 2
// [[2,2],[2,2]]
// 3
// [[1,2]]
// 1
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let shift_times = k as usize % mat[0].len();

        for m in mat.iter() {
            for i in 0..m.len() {
                let mut dest = i + 1 + shift_times;
                if dest % m.len() == 0 {
                    dest -= 1;
                } else {
                    dest = dest % m.len() - 1;
                }

                if m[i] != m[dest] {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2946_solution() {
        // assert!(Solution::are_similar(
        //     vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
        //     2
        // ));
        // assert!(Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3));
        assert!(!Solution::are_similar(vec![vec![1, 2]], 1));
    }
}
