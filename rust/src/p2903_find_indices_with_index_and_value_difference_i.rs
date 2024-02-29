// You are given a
// 0-indexed integer array nums having length n, an integer indexDifference, and an integer valueDifference.
// Your task is to find
// two indices i and j, both in the range [0, n - 1], that satisfy the following conditions:
//  abs(i - j) = indexDifference, and abs(nums[i] - nums[j]) = valueDifference  Return an integer array answer, where answer = [i, j] if there are two such indices, and answer = [-1, -1] otherwise. If there are multiple choices for the two indices, return any of them.
//
// Note: i and j may be
// equal.
//
// Example 1:
//
// Input: nums = [5,1,4,1], indexDifference = 2, valueDifference = 4
// Output: [0,3]
// Explanation: In this example, i = 0 and j = 3 can be selected. abs(0 - 3) = 2 and abs(nums[0] - nums[3]) = 4. Hence, a valid answer is [0,3]. [3,0] is also a valid answer.  Example 2:
//
// Input: nums = [2,1], indexDifference = 0, valueDifference = 0
// Output: [0,0]
// Explanation: In this example, i = 0 and j = 0 can be selected. abs(0 - 0) = 0 and abs(nums[0] - nums[0]) = 0. Hence, a valid answer is [0,0]. Other valid answers are [0,1], [1,0], and [1,1].  Example 3:
//
// Input: nums = [1,2,3], indexDifference = 2, valueDifference = 4
// Output: [-1,-1]
// Explanation: In this example, it can be shown that it is impossible to find two indices that satisfy both conditions. Hence, [-1,-1] is returned.
//
// Constraints:
//  1 = n == nums.length = 100 0 = nums[i] = 50 0 = indexDifference = 100 0 = valueDifference = 50

// hint 1
// Use bruteforce.

// hint 2
// You can use a nested loop to compare each pair of indices (i, j) and check if the conditions are satisfied.

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if i.abs_diff(j) as i32 >= index_difference
                    && nums[i].abs_diff(nums[j]) as i32 >= value_difference
                {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_2903_solution() {
        assert_eq!(vec![0, 3], Solution::find_indices(vec![5, 1, 4, 1], 2, 4));
        assert_eq!(vec![0, 0], Solution::find_indices(vec![2, 1], 0, 0));
        assert_eq!(vec![-1, -1], Solution::find_indices(vec![1, 2, 3], 2, 4));
    }
}
