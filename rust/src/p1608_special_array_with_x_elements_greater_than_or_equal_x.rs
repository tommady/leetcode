// You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x. Notice that x does not have to be an element in nums. Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique. Example 1: Input: nums = [3,5] Output: 2 Explanation: There are 2 values (3 and 5) that are greater than or equal to 2. Example 2: Input: nums = [0,0] Output: -1 Explanation: No numbers fit the criteria for x. If x = 0, there should be 0 numbers = x, but there are 2. If x = 1, there should be 1 number = x, but there are 0. If x = 2, there should be 2 numbers = x, but there are 0. x cannot be greater since there are only 2 numbers in nums. Example 3: Input: nums = [0,4,3,0,4] Output: 3 Explanation: There are 3 values that are greater than or equal to 3. Example 4: Input: nums = [3,6,7,7,0] Output: -1 Constraints: 1 = nums.length = 100 0 = nums[i] = 1000
//
//
// [3,5]
// [0,0]
// [0,4,3,0,4]
// [3,6,7,7,0]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        for x in 0..=nums.len() {
            let mut counter = 0;
            for num in nums.iter() {
                if (x as i32) <= *num {
                    counter += 1;
                }
            }
            if counter == x {
                return x as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1608_solution() {
        assert_eq!(2, Solution::special_array(vec![3, 5]));
        assert_eq!(-1, Solution::special_array(vec![0, 0]));
        assert_eq!(3, Solution::special_array(vec![0, 4, 3, 0, 4]));
        assert_eq!(-1, Solution::special_array(vec![3, 6, 7, 7, 0]));
    }
}
