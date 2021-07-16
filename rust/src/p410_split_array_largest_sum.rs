// Given an array nums which consists of non-negative integers and an integer m, you can split the array into m non-empty continuous subarrays. Write an algorithm to minimize the largest sum among these m subarrays. Example 1: Input: nums = [7,2,5,10,8], m = 2 Output: 18 Explanation: There are four ways to split nums into two subarrays. The best way is to split it into [7,2,5] and [10,8], where the largest sum among the two subarrays is only 18. Example 2: Input: nums = [1,2,3,4,5], m = 2 Output: 9 Example 3: Input: nums = [1,4,4], m = 3 Output: 4 Constraints: 1 = nums.length = 1000 0 = nums[i] = 106 1 = m = min(50, nums.length)
//
//
// [7,2,5,10,8]
// 2
// [1,2,3,4,5]
// 2
// [1,4,4]
// 3
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut max: i32 = 0;
        let mut min: i32 = 0;
        let mut ret: i32 = i32::MAX;

        for num in nums.iter() {
            min = *num.max(&min);
            max += num;
        }

        while min <= max {
            let mid: i32 = min + (max - min) / 2;
            let mut check: i32 = 0;
            let mut counter: i32 = 1;

            for num in nums.iter() {
                if check + *num > mid {
                    check = 0;
                    counter += 1;
                }
                check += num;
            }

            if counter <= m {
                ret = mid.min(ret);
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_410_solution() {
        assert_eq!(18, Solution::split_array(vec![7, 2, 5, 10, 8], 2));
        assert_eq!(9, Solution::split_array(vec![1, 2, 3, 4, 5], 2));
        assert_eq!(4, Solution::split_array(vec![1, 4, 4], 3));
    }
}
