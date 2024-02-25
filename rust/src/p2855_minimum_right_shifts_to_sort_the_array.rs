// You are given a
// 0-indexed array nums of length n containing
// distinct positive integers. Return the
// minimum number of
// right shifts required to sort nums and -1 if this is not possible.
// A
// right shift is defined as shifting the element at index i to index (i + 1) % n, for all indices.
//
// Example 1:
//
// Input: nums = [3,4,5,1,2]
// Output: 2
// Explanation: After the first right shift, nums = [2,3,4,5,1]. After the second right shift, nums = [1,2,3,4,5]. Now nums is sorted; therefore the answer is 2.  Example 2:
//
// Input: nums = [1,3,5]
// Output: 0
// Explanation: nums is already sorted therefore, the answer is 0. Example 3:
//
// Input: nums = [2,1,4]
// Output: -1
// Explanation: It's impossible to sort the array using right shifts.
//
// Constraints:
//  1 = nums.length = 100 1 = nums[i] = 100 nums contains distinct integers.

// hint 1
// Find the pivot point around which the array is rotated.

// hint 2
// Will the answer exist if there is more than one point where nums[i] ?

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] > nums[(i + 1) % nums.len()] {
                count += 1;
                index = i + 1;
            }
        }
        if count > 1 {
            return -1;
        }
        if count == 0 {
            return 0;
        }
        (nums.len() - index) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_2855_solution() {
        assert_eq!(2, Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]));
        assert_eq!(0, Solution::minimum_right_shifts(vec![1, 3, 5]));
        assert_eq!(-1, Solution::minimum_right_shifts(vec![2, 1, 4]));
        assert_eq!(
            -1,
            Solution::minimum_right_shifts(vec![92, 84, 37, 19, 16, 85, 20, 79, 25, 89])
        );
        assert_eq!(
            -1,
            Solution::minimum_right_shifts(vec![63, 51, 65, 87, 6, 17, 32, 14, 42, 46])
        );
        assert_eq!(
            -1,
            Solution::minimum_right_shifts(vec![4, 33, 17, 3, 8, 91, 28, 13, 72, 42])
        );
        assert_eq!(
            -1,
            Solution::minimum_right_shifts(vec![12, 28, 63, 56, 69, 67, 83, 37, 82, 70])
        );
        assert_eq!(-1, Solution::minimum_right_shifts(vec![29, 30, 88, 28, 62]));
    }
}
