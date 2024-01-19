// You are given an array of distinct integers arr and an array of integer arrays pieces, where the integers in pieces are distinct. Your goal is to form arr by concatenating the arrays in pieces in any order. However, you are not allowed to reorder the integers in each array pieces[i]. Return true if it is possible to form the array arr from pieces. Otherwise, return false. Example 1: Input: arr = [85], pieces = [[85]] Output: true Example 2: Input: arr = [15,88], pieces = [[88],[15]] Output: true Explanation: Concatenate [15] then [88] Example 3: Input: arr = [49,18,16], pieces = [[16,18,49]] Output: false Explanation: Even though the numbers match, we cannot reorder pieces[0]. Example 4: Input: arr = [91,4,64,78], pieces = [[78],[4,64],[91]] Output: true Explanation: Concatenate [91] then [4,64] then [78] Example 5: Input: arr = [1,3,5,7], pieces = [[2,4,6,8]] Output: false Constraints: 1 = pieces.length = arr.length = 100 sum(pieces[i].length) == arr.length 1 = pieces[i].length = arr.length 1 = arr[i], pieces[i][j] = 100 The integers in arr are distinct. The integers in pieces are distinct (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).
//
//
// [85]
// [[85]]
// [15,88]
// [[88],[15]]
// [49,18,16]
// [[16,18,49]]
// [91,4,64,78]
// [[78],[4,64],[91]]
// [1,3,5,7]
// [[2,4,6,8]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;

        while i < arr.len() {
            let mut j = 0;

            // locating the pieces
            while j < pieces.len() {
                if arr[i] == pieces[j][0] {
                    break;
                }
                j += 1;
            }

            // means not found
            if j == pieces.len() {
                return false;
            }

            let mut k = 0;
            while k < pieces[j].len() {
                if arr[i] != pieces[j][k] {
                    return false;
                }
                i += 1;
                k += 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1640_solution() {
        assert!(Solution::can_form_array(vec![85], vec![vec![85]]));
        assert!(Solution::can_form_array(
            vec![15, 88],
            vec![vec![88], vec![15]]
        ));
        assert!(!Solution::can_form_array(
            vec![49, 18, 16],
            vec![vec![16, 18, 49]]
        ));
        assert!(Solution::can_form_array(
            vec![91, 4, 64, 78],
            vec![vec![78], vec![4, 64], vec![91]]
        ));
        assert!(!Solution::can_form_array(
            vec![1, 3, 5, 7],
            vec![vec![2, 4, 6, 8]]
        ));
        assert!(!Solution::can_form_array(
            vec![1, 2, 3],
            vec![vec![1], vec![3, 2]]
        ));
    }
}
