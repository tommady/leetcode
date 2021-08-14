// Given an unsorted integer array nums, return the smallest missing positive integer. You must implement an algorithm that runs in O(n) time and uses constant extra space. Example 1: Input: nums = [1,2,0] Output: 3 Example 2: Input: nums = [3,4,-1,1] Output: 2 Example 3: Input: nums = [7,8,9,11,12] Output: 1 Constraints: 1 = nums.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 105 -231 = nums[i] = 231 - 1
//
//
// [1,2,0]
// [3,4,-1,1]
// [7,8,9,11,12]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            if nums[i] <= 0 || nums[i] as usize > n {
                nums[i] = n as i32 + 1;
            }
        }

        for i in 0..n {
            let num = (nums[i].abs() - 1) as usize;
            if num < n && nums[num] > 0 {
                nums[num] = -1 * nums[num];
            }
        }

        for i in 0..n {
            if nums[i] > 0 {
                return (i + 1) as i32;
            }
        }

        return (n + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41_solution() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }
}
