// You are given an integer array nums (0-indexed). In one operation, you can choose an element of the array and increment it by 1.   For example, if nums = [1,2,3], you can choose to increment nums[1] to make nums = [1,3,3].   Return the minimum number of operations needed to make nums strictly increasing.  An array nums is strictly increasing if nums[i] nums[i+1] for all 0 = i nums.length - 1. An array of length 1 is trivially strictly increasing.   Example 1:   Input: nums = [1,1,1] Output: 3 Explanation: You can do the following operations: 1) Increment nums[2], so nums becomes [1,1,2]. 2) Increment nums[1], so nums becomes [1,2,2]. 3) Increment nums[2], so nums becomes [1,2,3].   Example 2:   Input: nums = [1,5,2,4,1] Output: 14   Example 3:   Input: nums = [8] Output: 0    Constraints:   1 = nums.length = 5000 1 = nums[i] = 104
//
//
// [1,1,1]
// [1,5,2,4,1]
// [8]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut counter = 0;

        fn abs(v: i32) -> i32 {
            if v < 0 {
                return -v;
            }
            v
        }

        let mut i = 1;
        while i < nums.len() {
            if nums[i - 1] >= nums[i] {
                let ops = abs(nums[i] - nums[i - 1]) + 1;
                nums[i] += ops;
                counter += ops;
            }
            i += 1;
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1827_solution() {
        assert_eq!(3, Solution::min_operations(vec![1, 1, 1]));
        // [1, 5, 2, 4, 1]
        // [1, 5, 6, 4, 1] + 4
        // [1, 5, 6, 7, 1] + 4 + 3
        // [1, 5, 6, 7, 8] + 4 + 3 + 7
        assert_eq!(14, Solution::min_operations(vec![1, 5, 2, 4, 1]));
        assert_eq!(0, Solution::min_operations(vec![8]));
    }
}
