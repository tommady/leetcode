// Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i j k and nums[i] nums[j] nums[k]. If no such indices exists, return false. Example 1: Input: nums = [1,2,3,4,5] Output: true Explanation: Any triplet where i j k is valid. Example 2: Input: nums = [5,4,3,2,1] Output: false Explanation: No triplet exists. Example 3: Input: nums = [2,1,5,0,4,6] Output: true Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 nums[4] == 4 nums[5] == 6. Constraints: 1 = nums.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 105 -231 = nums[i] = 231 - 1 Follow up: Could you implement a solution that runs in O(n) time complexity and O(1) space complexity?
//
//
// [1,2,3,4,5]
// [5,4,3,2,1]
// [2,1,5,0,4,6]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut a = i32::MAX;
        let mut b = i32::MAX;

        for num in nums {
            if a >= num {
                a = num
            } else if b >= num {
                b = num
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_334_solution() {
        assert_eq!(true, Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
        assert_eq!(true, Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
        assert_eq!(
            true,
            Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13])
        );
    }
}
