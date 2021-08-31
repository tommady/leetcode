// Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal. In one move, you can increment n - 1 elements of the array by 1. Example 1: Input: nums = [1,2,3] Output: 3 Explanation: Only three moves are needed (remember each move increments two elements): [1,2,3] = [2,3,3] = [3,4,3] = [4,4,4] Example 2: Input: nums = [1,1,1] Output: 0 Constraints: n == nums.length 1 = nums.length = 105 -109 = nums[i] = 109 The answer is guaranteed to fit in a 32-bit integer.
//
//
// [1,2,3]
// [1,1,1]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_moves(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        nums.sort_unstable();

        let mut i = nums.len() - 1;
        while i > 0 {
            ret += nums[i] - nums[0];
            i -= 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_453_solution() {
        assert_eq!(3, Solution::min_moves(vec![1, 2, 3]));
        assert_eq!(0, Solution::min_moves(vec![1, 1, 1]));
    }
}
