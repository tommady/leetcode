// You are given a 0-indexed array of integers nums. A prefix nums[0..i] is sequential if, for all 1 = j = i, nums[j] = nums[j - 1] + 1. In particular, the prefix consisting only of nums[0] is sequential. Return the smallest integer x missing from nums such that x is greater than or equal to the sum of the longest sequential prefix. Example 1: Input: nums = [1,2,3,2,5] Output: 6 Explanation: The longest sequential prefix of nums is [1,2,3] with a sum of 6. 6 is not in the array, therefore 6 is the smallest missing integer greater than or equal to the sum of the longest sequential prefix. Example 2: Input: nums = [3,4,5,1,12,14,13] Output: 15 Explanation: The longest sequential prefix of nums is [3,4,5] with a sum of 12. 12, 13, and 14 belong to the array while 15 does not. Therefore 15 is the smallest missing integer greater than or equal to the sum of the longest sequential prefix. Constraints: 1 = nums.length = 50 1 = nums[i] = 50
//
//
// [1,2,3,2,5]
// [3,4,5,1,12,14,13]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::with_capacity(nums.len());
        m.insert(nums[0], true);
        let mut ret = nums[0];
        let mut goon = true;

        for i in 1..nums.len() {
            m.insert(nums[i], true);
            if nums[i] - 1 == nums[i - 1] && goon {
                ret += nums[i]
            } else {
                goon = false;
            }
        }

        while let Some(_num) = m.get(&ret) {
            ret += 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2996_solution() {
        assert_eq!(6, Solution::missing_integer(vec![1, 2, 3, 2, 5]));
        assert_eq!(15, Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]));
        assert_eq!(
            297,
            Solution::missing_integer(vec![29, 30, 31, 32, 33, 34, 35, 36, 37])
        );
        assert_eq!(
            38,
            Solution::missing_integer(vec![37, 1, 2, 9, 5, 8, 5, 2, 9, 4])
        );
        assert_eq!(39, Solution::missing_integer(vec![38, 43, 44]));
    }
}
