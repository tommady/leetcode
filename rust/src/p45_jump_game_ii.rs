// Given an array of non-negative integers nums, you are initially positioned at the first index of the array. Each element in the array represents your maximum jump length at that position. Your goal is to reach the last index in the minimum number of jumps. You can assume that you can always reach the last index. Example 1: Input: nums = [2,3,1,1,4] Output: 2 Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index. Example 2: Input: nums = [2,3,0,1,4] Output: 2 Constraints: 1 = nums.length = 104 0 = nums[i] = 1000
//
//
// [2,3,1,1,4]
// [2,3,0,1,4]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut ret = 0;

        for (i, num) in nums.iter().enumerate().take(nums.len() - 1) {
            if y < i {
                break;
            }

            y = y.max(i + *num as usize);
            if x == i {
                x = y;
                ret += 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45_solution() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
        assert_eq!(1, Solution::jump(vec![2, 1]));
        assert_eq!(0, Solution::jump(vec![1]));
        assert_eq!(2, Solution::jump(vec![1, 2, 3]));
        assert_eq!(1, Solution::jump(vec![1, 2]));
        assert_eq!(3, Solution::jump(vec![1, 1, 1, 1]));
        assert_eq!(2, Solution::jump(vec![1, 2, 0, 1]));
        assert_eq!(
            8,
            Solution::jump(vec![
                9, 4, 5, 4, 1, 8, 1, 2, 0, 7, 8, 7, 0, 6, 6, 1, 1, 2, 5, 0, 9, 8, 4, 7, 9, 6, 8, 1,
                4, 0, 8, 5, 5, 3, 9, 8, 1, 2, 2, 3, 0, 1, 3, 2, 7, 9, 3, 0, 1
            ])
        );
    }
}
