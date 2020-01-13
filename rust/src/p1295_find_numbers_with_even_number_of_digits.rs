// Given an array nums of integers, return how many of them contain an even number of digits.
//
// Example 1:
//
//
// Input: nums = [12,345,2,6,7896]
// Output: 2
// Explanation:
// 12 contains 2 digits (even number of digits).
// 345 contains 3 digits (odd number of digits).
// 2 contains 1 digit (odd number of digits).
// 6 contains 1 digit (odd number of digits).
// 7896 contains 4 digits (even number of digits).
// Therefore only 12 and 7896 contain an even number of digits.
//
//
// Example 2:
//
//
// Input: nums = [555,901,482,1771]
// Output: 1
// Explanation:
// Only 1771 contains an even number of digits.
//
//
//
// Constraints:
//
//
// 	1 = nums.length = 500
// 	1 = nums[i] = 10^5
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ret: i32 = 0;
        for n in nums.into_iter() {
            let s = n.to_string();
            if s.len() % 2 == 0 {
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
    fn test_1295_solution() {
        assert_eq!(1, Solution::find_numbers(vec![555, 901, 482, 1771]))
    }
}
