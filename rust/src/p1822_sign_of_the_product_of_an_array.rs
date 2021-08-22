// There is a function signFunc(x) that returns: 1 if x is positive. -1 if x is negative. 0 if x is equal to 0. You are given an integer array nums. Let product be the product of all values in the array nums. Return signFunc(product). Example 1: Input: nums = [-1,-2,-3,-4,3,2,1] Output: 1 Explanation: The product of all values in the array is 144, and signFunc(144) = 1 Example 2: Input: nums = [1,5,0,2,-3] Output: 0 Explanation: The product of all values in the array is 0, and signFunc(0) = 0 Example 3: Input: nums = [-1,1,-1,1,-1] Output: -1 Explanation: The product of all values in the array is -1, and signFunc(-1) = -1 Constraints: 1 = nums.length = 1000 -100 = nums[i] = 100
//
//
// [-1,-2,-3,-4,3,2,1]
// [1,5,0,2,-3]
// [-1,1,-1,1,-1]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut nag_counter = 0;

        for num in nums {
            if num == 0 {
                return 0;
            }

            if num < 0 {
                nag_counter += 1;
            }
        }

        if nag_counter % 2 != 0 {
            return -1;
        }

        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1822_solution() {
        assert_eq!(1, Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));
        assert_eq!(0, Solution::array_sign(vec![1, 5, 0, 2, -3]));
        assert_eq!(-1, Solution::array_sign(vec![-1, 1, -1, 1, -1]));
        assert_eq!(
            -1,
            Solution::array_sign(vec![
                41, 65, 14, 80, 20, 10, 55, 58, 24, 56, 28, 86, 96, 10, 3, 84, 4, 41, 13, 32, 42,
                43, 83, 78, 82, 70, 15, -41
            ])
        );
    }
}
