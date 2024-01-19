// Given an integer n (in base 10) and a base k, return the sum of the digits of n after converting n from base 10 to base k. After converting, each digit should be interpreted as a base 10 number, and the sum should be returned in base 10. Example 1: Input: n = 34, k = 6 Output: 9 Explanation: 34 (base 10) expressed in base 6 is 54. 5 + 4 = 9. Example 2: Input: n = 10, k = 10 Output: 1 Explanation: n is already in base 10. 1 + 0 = 1. Constraints: 1 = n = 100 2 = k = 10
//
//
// 34
// 6
// 10
// 10
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut ret = 0;

        while n > 0 {
            ret += n % k;
            n /= k;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1837_solution() {
        assert_eq!(9, Solution::sum_base(34, 6));
        assert_eq!(1, Solution::sum_base(10, 10));
        assert_eq!(2, Solution::sum_base(68, 2));
    }
}
