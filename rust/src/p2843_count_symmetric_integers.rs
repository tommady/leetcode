// You are given two positive integers low and high. An integer x consisting of 2 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n digits is symmetric if the sum of the first n digits of x is equal to the sum of the last n digits of x. Numbers with an odd number of digits are never symmetric. Return the number of symmetric integers in the range [low, high]. Example 1: Input: low = 1, high = 100 Output: 9 Explanation: There are 9 symmetric integers between 1 and 100: 11, 22, 33, 44, 55, 66, 77, 88, and 99. Example 2: Input: low = 1200, high = 1230 Output: 4 Explanation: There are 4 symmetric integers between 1200 and 1230: 1203, 1212, 1221, and 1230. Constraints: 1 = low = high = 104
//
//
// 1
// 100
// 1200
// 1230
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        const RADIX: u32 = 10;
        let mut ret = 0;

        for num in low..high + 1 {
            let num = num as u32;
            let n = num.checked_ilog10().unwrap_or(0) + 1;
            if n % 2 != 0 {
                continue;
            }

            let left = num / 10_u32.pow(n / 2);
            let left_sum = left
                .to_string()
                .chars()
                .map(|c| c.to_digit(RADIX).unwrap_or(0))
                .sum::<u32>();
            let right_sum = (num - left * 10_u32.pow(n / 2))
                .to_string()
                .chars()
                .map(|c| c.to_digit(RADIX).unwrap_or(0))
                .sum::<u32>();
            if left_sum == right_sum {
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
    fn test_2843_solution() {
        // assert_eq!(9, Solution::count_symmetric_integers(1, 100));
        // assert_eq!(4, Solution::count_symmetric_integers(1200, 1230));
        assert_eq!(44, Solution::count_symmetric_integers(100, 1782));
    }
}
