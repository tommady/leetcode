// You are given an integer array nums containing
// positive integers. We define a function encrypt such that encrypt(x) replaces
// every digit in x with the
// largest digit in x. For example, encrypt(523) = 555 and encrypt(213) = 333.
// Return the
// sum of encrypted elements.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 6
//
// Explanation: The encrypted elements are [1,2,3]. The sum of encrypted elements is 1 + 2 + 3 == 6.
//  Example 2:
//
// Input: nums = [10,21,31]
//
// Output: 66
//
// Explanation: The encrypted elements are [11,22,33]. The sum of encrypted elements is 11 + 22 + 33 == 66.
//
//
// Constraints:
//  1 = nums.length = 50 1 = nums[i] = 1000

// hint 1
// Encrypted numbers are of the form 11…1 Cargo.lock Cargo.toml Justfile rustfmt.toml src target template.rs maxDigit.

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for mut num in nums {
            let (mut count, mut max) = (0, 0);
            while num != 0 {
                count += 1;
                max = max.max(num % 10);
                num /= 10;
            }

            for _ in 0..count {
                ret += max;
                max *= 10;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_3079_solution() {
        assert_eq!(6, Solution::sum_of_encrypted_int(vec![1, 2, 3]));
        assert_eq!(66, Solution::sum_of_encrypted_int(vec![10, 21, 31]));
    }
}
