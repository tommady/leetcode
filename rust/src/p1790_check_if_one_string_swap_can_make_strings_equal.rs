// You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
//
// Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.
//
//
// Example 1:
//
//
// Input: s1 = bank , s2 = kanb
// Output: true
// Explanation: For example, swap the first character with the last character of s2 to make bank .
//
//
// Example 2:
//
//
// Input: s1 = attack , s2 = defend
// Output: false
// Explanation: It is impossible to make them equal with one string swap.
//
//
// Example 3:
//
//
// Input: s1 = kelb , s2 = kelb
// Output: true
// Explanation: The two strings are already equal, so no string swap operation is required.
//
//
// Example 4:
//
//
// Input: s1 = abcd , s2 = dcba
// Output: false
//
//
//
// Constraints:
//
//
// 	1 = s1.length, s2.length = 100
// 	s1.length == s2.length
// 	s1 and s2 consist of only lowercase English letters.
//
//
// "bank"
// "kanb"
// "attack"
// "defend"
// "kelb"
// "kelb"
// "abcd"
// "dcba"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }

        let bs1 = s1.as_bytes();
        let bs2 = s2.as_bytes();
        let mut i = 0;
        let mut swap1_1: i32 = -1;
        let mut swap1_2: i32 = -1;
        let mut swap2_1: i32 = -1;
        let mut swap2_2: i32 = -1;

        while i < bs2.len() {
            if bs1[i] != bs2[i] {
                if swap1_1 == -1 && swap1_2 == -1 {
                    swap1_1 = bs1[i] as i32;
                    swap1_2 = bs2[i] as i32;
                } else if swap2_1 == -1 && swap2_2 == -1 {
                    swap2_1 = bs1[i] as i32;
                    swap2_2 = bs2[i] as i32;
                } else {
                    return false;
                }
            }

            i += 1;
        }

        swap1_1 == swap2_2 && swap1_2 == swap2_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1790_solution() {
        assert!(Solution::are_almost_equal(
            "bank".to_string(),
            "kanb".to_string()
        ));
        assert!(!Solution::are_almost_equal(
            "attack".to_string(),
            "defend".to_string()
        ));
        assert!(Solution::are_almost_equal(
            "kelb".to_string(),
            "kelb".to_string()
        ));

        assert!(!Solution::are_almost_equal(
            "abcd".to_string(),
            "dcba".to_string()
        ));
        assert!(Solution::are_almost_equal(
            "siyolsdcjthwsiplccjbuceoxmpjgrauocx".to_string(),
            "siyolsdcjthwsiplccpbuceoxmjjgrauocx".to_string()
        ));
    }
}
