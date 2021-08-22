// Given an integer array nums, find three numbers whose product is maximum and return the maximum product. Example 1: Input: nums = [1,2,3] Output: 6 Example 2: Input: nums = [1,2,3,4] Output: 24 Example 3: Input: nums = [-1,-2,-3] Output: -6 Constraints: 3 = nums.length = 104 -1000 = nums[i] = 1000
//
//
// [1,2,3]
// [1,2,3,4]
// [-1,-2,-3]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut max3 = i32::MIN;
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;

        for num in nums {
            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }

            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }
        }

        (max1 * max2 * max3).max(min1 * min2 * max1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_628_solution() {
        assert_eq!(6, Solution::maximum_product(vec![1, 2, 3]));
        assert_eq!(24, Solution::maximum_product(vec![1, 2, 3, 4]));
        assert_eq!(-6, Solution::maximum_product(vec![-1, -2, -3]));
        assert_eq!(
            39200,
            Solution::maximum_product(vec![-100, -98, -1, 2, 3, 4])
        );
    }
}
