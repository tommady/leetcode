// A ramp in an integer array nums is a pair (i, j) for which i j and nums[i] = nums[j]. The width of such a ramp is j - i. Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0. Example 1: Input: nums = [6,0,8,2,1,5] Output: 4 Explanation: The maximum width ramp is achieved at (i, j) = (1, 5): nums[1] = 0 and nums[5] = 5. Example 2: Input: nums = [9,8,1,0,1,9,4,0,4,1] Output: 7 Explanation: The maximum width ramp is achieved at (i, j) = (2, 9): nums[2] = 1 and nums[9] = 1. Constraints: 2 = nums.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 104 0 = nums[i] = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 104
//
//
// [6,0,8,2,1,5]
// [9,8,1,0,1,9,4,0,4,1]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_width_ramp(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let n = nums.len();
        let mut i = n - 2;

        loop {
            while i + ret + 1 < n && nums[i] <= nums[i + ret + 1] {
                ret += 1;
            }
            nums[i] = nums[i].max(nums[i + 1]);

            if i != 0 {
                i -= 1;
            } else {
                break;
            }
        }

        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_962_solution() {
        assert_eq!(4, Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]));
        assert_eq!(
            7,
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
        );
    }
}
