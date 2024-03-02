// Given a positive integer n, find the sum of all integers in the range [1, n]
// inclusive that are divisible by 3, 5, or 7.
// Return an integer denoting the sum of all numbers in the given range satisfying the constraint.
//
// Example 1:
//
// Input: n = 7
// Output: 21
// Explanation: Numbers in the range [1, 7] that are divisible by 3, 5, or 7 are 3, 5, 6, 7. The sum of these numbers is 21.  Example 2:
//
// Input: n = 10
// Output: 40
// Explanation: Numbers in the range [1, 10] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9, 10. The sum of these numbers is 40.  Example 3:
//
// Input: n = 9
// Output: 30
// Explanation: Numbers in the range [1, 9] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9. The sum of these numbers is 30.
//
// Constraints:
//  1 = n = 103

// hint 1
// Iterate through the range 1 to n and count numbers divisible by either 3, 5, or 7.

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut ret = 0;
        for i in 1..n + 1 {
            if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
                ret += i;
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
    fn test_2652_solution() {
        assert_eq!(21, Solution::sum_of_multiples(7));
        assert_eq!(40, Solution::sum_of_multiples(10));
        assert_eq!(30, Solution::sum_of_multiples(9));
    }
}
