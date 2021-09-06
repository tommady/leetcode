// Given an array nums, you are allowed to choose one element of nums and change it by any value in one move.  Return the minimum difference between the largest and smallest value of nums after perfoming at most 3 moves.   Example 1:   Input: nums = [5,3,2,4] Output: 0 Explanation: Change the array [5,3,2,4] to [2,2,2,2]. The difference between the maximum and minimum is 2-2 = 0.  Example 2:   Input: nums = [1,5,0,10,14] Output: 1 Explanation: Change the array [1,5,0,10,14] to [1,1,0,1,1].  The difference between the maximum and minimum is 1-0 = 1.   Example 3:   Input: nums = [6,6,0,1,1,4,6] Output: 2   Example 4:   Input: nums = [1,5,6,14,15] Output: 1    Constraints:   1 = nums.length = 10^5 -10^9 = nums[i] = 10^9
//
//
// [5,3,2,4]
// [1,5,0,10,14]
// [6,6,0,1,1,4,6]
// [1,5,6,14,15]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            return 0;
        }

        nums.sort_unstable();
        let a = (nums[0] - nums[nums.len() - 4]).abs();
        let b = (nums[3] - nums[nums.len() - 1]).abs();
        let c = (nums[1] - nums[nums.len() - 3]).abs();
        let d = (nums[2] - nums[nums.len() - 2]).abs();

        a.min(b.min(c.min(d)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1509_solution() {
        assert_eq!(0, Solution::min_difference(vec![5, 3, 2, 4]));
        assert_eq!(1, Solution::min_difference(vec![1, 5, 0, 10, 14]));
        assert_eq!(2, Solution::min_difference(vec![6, 6, 0, 1, 1, 4, 6]));
        assert_eq!(1, Solution::min_difference(vec![1, 5, 6, 14, 15]));
        assert_eq!(2, Solution::min_difference(vec![1, 3]));
    }
}
