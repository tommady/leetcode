// Given an array of distinct integers arr, find all pairs of elements with the minimum absolute difference of any two elements. Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows a, b are from arr a b b - a equals to the minimum absolute difference of any two elements in arr Example 1: Input: arr = [4,2,1,3] Output: [[1,2],[2,3],[3,4]] Explanation: The minimum absolute difference is 1. List all pairs with difference equal to 1 in ascending order. Example 2: Input: arr = [1,3,6,10,15] Output: [[1,3]] Example 3: Input: arr = [3,8,-10,23,19,-4,-14,27] Output: [[-14,-10],[19,23],[23,27]] Constraints: 2 = arr.length = 10^5 -10^6 = arr[i] = 10^6
//
//
// [4,2,1,3]
// [1,3,6,10,15]
// [3,8,-10,23,19,-4,-14,27]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut the_min_diff: i32 = i32::MAX;
        let mut ret = Vec::new();

        for i in 1..arr.len() {
            let diff = (arr[i] - arr[i - 1]).abs();

            if diff < the_min_diff {
                the_min_diff = diff;
                ret.clear();
                ret.push(vec![arr[i - 1], arr[i]]);
            } else if diff == the_min_diff {
                ret.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1200_solution() {
        assert_eq!(
            vec![vec![1, 2], vec![2, 3], vec![3, 4]],
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
        );
        assert_eq!(
            vec![vec![1, 3]],
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
        );
        assert_eq!(
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
        );
        assert_eq!(
            vec![vec![-25, -23]],
            Solution::minimum_abs_difference(vec![-25, -51, -29, -23, 57, -18]),
        )
    }
}
