// Given an array nums, you are allowed to choose one element of nums and change it by any value in one move.
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