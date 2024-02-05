// You are given three strings s1, s2, and s3. You have to perform the following operation on these three strings as many times as you want. In one operation you can choose one of these three strings such that its length is at least 2 and delete the rightmost character of it. Return the minimum number of operations you need to perform to make the three strings equal if there is a way to make them equal, otherwise, return -1. Example 1: Input: s1 = abc , s2 = abb , s3 = ab Output: 2 Explanation: Performing operations on s1 and s2 once will lead to three equal strings. It can be shown that there is no way to make them equal with less than two operations. Example 2: Input: s1 = dac , s2 = bac , s3 = cac Output: -1 Explanation: Because the leftmost letters of s1 and s2 are not equal, they could not be equal after any number of operations. So the answer is -1. Constraints: 1 = s1.length, s2.length, s3.length = 100 s1, s2 and s3 consist only of lowercase English letters.
//
//
// "abc"
// "abb"
// "ab"
// "dac"
// "bac"
// "cac"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let (s1_bytes, s2_bytes, s3_bytes) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());

        if s1_bytes[0] != s2_bytes[0] || s2_bytes[0] != s3_bytes[0] || s3_bytes[0] != s1_bytes[0] {
            return -1;
        }

        let (s1_len, s2_len, s3_len) = (s1.len(), s2.len(), s3.len());
        let min_len = s1_len.min(s2_len).min(s3_len);

        let mut min_diff = 0;
        let mut ret =
            (s1_len.abs_diff(min_len) + s2_len.abs_diff(min_len) + s3_len.abs_diff(min_len)) as i32;
        for i in 1..min_len {
            if s1_bytes[i] != s2_bytes[i]
                || s2_bytes[i] != s3_bytes[i]
                || s3_bytes[i] != s1_bytes[i]
            {
                min_diff = i;
                break;
            }
        }

        if min_diff != 0 {
            ret = (s1_len.abs_diff(min_diff)
                + s2_len.abs_diff(min_diff)
                + s3_len.abs_diff(min_diff)) as i32
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2937_solution() {
        assert_eq!(
            2,
            Solution::find_minimum_operations(
                "abc".to_string(),
                "abb".to_string(),
                "ab".to_string()
            )
        );
        assert_eq!(
            -1,
            Solution::find_minimum_operations(
                "dac".to_string(),
                "bac".to_string(),
                "cac".to_string()
            )
        );
        assert_eq!(
            10,
            Solution::find_minimum_operations(
                "bcbb".to_string(),
                "bccbabb".to_string(),
                "bcabb".to_string()
            )
        );
    }
}
