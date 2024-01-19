// Given an alphanumeric string s, return the second largest numerical digit that appears in s, or -1 if it does not exist. An alphanumeric string is a string consisting of lowercase English letters and digits. Example 1: Input: s = dfa12321afd Output: 2 Explanation: The digits that appear in s are [1, 2, 3]. The second largest digit is 2. Example 2: Input: s = abc1111 Output: -1 Explanation: The digits that appear in s are [1]. There is no second largest digit. Constraints: 1 = s.length = 500 s consists of only lowercase English letters and/or digits.
//
//
// "dfa12321afd"
// "abc1111"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut first_high: i32 = -1;
        let mut second_high: i32 = -1;

        for b in s.as_bytes() {
            let mut n = *b as i32 - 57;
            if n <= 0 {
                n += 9;

                if n > first_high {
                    second_high = first_high;
                    first_high = n;
                } else if n != first_high && n > second_high {
                    second_high = n;
                }
            }
        }

        second_high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1796_solution() {
        assert_eq!(2, Solution::second_highest("dfa12321afd".to_string()));
        assert_eq!(-1, Solution::second_highest("abc1111".to_string()));
        assert_eq!(4, Solution::second_highest("sjhtz8344".to_string()));
        assert_eq!(7, Solution::second_highest("vwkxfq9791769".to_string()));
    }
}
