// You are given an array nums consisting of positive integers. Return the total frequencies of elements in nums such that those elements all have the maximum frequency. The frequency of an element is the number of occurrences of that element in the array. Example 1: Input: nums = [1,2,2,3,1,4] Output: 4 Explanation: The elements 1 and 2 have a frequency of 2 which is the maximum frequency in the array. So the number of elements in the array with maximum frequency is 4. Example 2: Input: nums = [1,2,3,4,5] Output: 5 Explanation: All elements of the array have a frequency of 1 which is the maximum. So the number of elements in the array with maximum frequency is 5. Constraints: 1 = nums.length = 100 1 = nums[i] = 100
//
//
// [1,2,2,3,1,4]
// [1,2,3,4,5]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut m = [0; 101];
        for num in &nums {
            let i = *num as usize;
            m[i] += 1;
        }

        let max = *m.iter().max().unwrap_or(&0);
        if max == 1 {
            return nums.len() as i32;
        }

        let mut ret = 0;
        for appears in &m {
            if *appears == max {
                ret += max
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3005_solution() {
        assert_eq!(4, Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]));
        assert_eq!(5, Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]));
    }
}
