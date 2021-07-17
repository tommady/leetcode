// Given an array of non-negative integers nums, you are initially positioned at the first index of the array. Each element in the array represents your maximum jump length at that position. Determine if you are able to reach the last index. Example 1: Input: nums = [2,3,1,1,4] Output: true Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index. Example 2: Input: nums = [3,2,1,0,4] Output: false Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index. Constraints: 1 = nums.length = 104 0 = nums[i] = 105
//
//
// [2,3,1,1,4]
// [3,2,1,0,4]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut position = 0;

        for i in 0..nums.len() {
            if i > position {
                return false;
            }

            let reached = i + nums[i] as usize;
            if position < reached {
                position = reached;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55_solution() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
