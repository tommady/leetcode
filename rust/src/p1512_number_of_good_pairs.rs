// Given an array of integers nums, return the number of good pairs. A pair (i, j) is called good if nums[i] == nums[j] and i j. Example 1: Input: nums = [1,2,3,1,1,3] Output: 4 Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed. Example 2: Input: nums = [1,1,1,1] Output: 6 Explanation: Each pair in the array are good. Example 3: Input: nums = [1,2,3] Output: 0 Constraints: 1 = nums.length = 100 1 = nums[i] = 100
//
//
// [1,2,3,1,1,3]
// [1,1,1,1]
// [1,2,3]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::new();

        for num in nums {
            m.entry(num)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut ret = 0;
        for v in m.values() {
            ret += v * (v - 1) / 2;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1512_solution() {
        assert_eq!(4, Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
        assert_eq!(6, Solution::num_identical_pairs(vec![1, 1, 1, 1]));
        assert_eq!(0, Solution::num_identical_pairs(vec![1, 2, 3]));
    }
}
