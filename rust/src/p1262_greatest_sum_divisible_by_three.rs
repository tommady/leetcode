// Given an array nums of integers, we need to find the maximum possible sum of elements of the array such that it is divisible by three. Example 1: Input: nums = [3,6,5,1,8] Output: 18 Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3). Example 2: Input: nums = [4] Output: 0 Explanation: Since 4 is not divisible by 3, do not pick any number. Example 3: Input: nums = [1,2,3,4,4] Output: 12 Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3). Constraints: 1 = nums.length = 4 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 10^4 1 = nums[i] = 10^4
//
//
// [3,6,5,1,8]
// [4]
// [1,2,3,4,4]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut m: [i32; 3] = [0; 3];
        m[1] = 60000;
        m[2] = 60000;

        fn min(a: i32, b: i32) -> i32 {
            if a < b {
                return a;
            }
            b
        }

        let mut sum: i32 = 0;

        for num in nums {
            if num % 3 == 1 {
                m[2] = min(m[2], m[1] + num);
                m[1] = min(m[1], num);
            } else if num % 3 == 2 {
                m[1] = min(m[1], m[2] + num);
                m[2] = min(m[2], num);
            }

            sum += num;
        }

        sum - m[(sum % 3) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1262_solution() {
        assert_eq!(18, Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]));
        assert_eq!(0, Solution::max_sum_div_three(vec![4]));
        assert_eq!(12, Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]));
    }
}
