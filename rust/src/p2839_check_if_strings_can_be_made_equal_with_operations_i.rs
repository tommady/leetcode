// You are given two strings s1 and s2, both of length 4, consisting of
// lowercase English letters.
// You can apply the following operation on any of the two strings
// any number of times:
//  Choose any two indices i and j such that j - i = 2, then
// swap the two characters at those indices in the string.  Return true if you can make the strings s1 and s2 equal, and false otherwise.
//
// Example 1:
//
// Input: s1 = "abcd", s2 = "cdab"
// Output: true
// Explanation: We can do the following operations on s1: - Choose the indices i = 0, j = 2. The resulting string is s1 = "cbad". - Choose the indices i = 1, j = 3. The resulting string is s1 = "cdab" = s2.  Example 2:
//
// Input: s1 = "abcd", s2 = "dacb"
// Output: false
// Explanation: It is not possible to make the two strings equal.
//
// Constraints:
//  s1.length == s2.length == 4 s1 and s2 consist only of lowercase English letters.

// hint 1
// Since the strings are very small you can try a brute-force approach.

// hint 2
// There are only 2 different swaps that are possible in a string.
#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        for i in 0..2 {
            if s1.as_bytes()[i] != s2.as_bytes()[i]
                && (s1.as_bytes()[i] != s2.as_bytes()[i + 2]
                    || s1.as_bytes()[i + 2] != s2.as_bytes()[i])
            {
                return false;
            }
        }
        for i in (2..4).rev() {
            if s1.as_bytes()[i] != s2.as_bytes()[i] && s1.as_bytes()[i] != s2.as_bytes()[i - 2] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_2839_solution() {
        assert_eq!(
            true,
            Solution::can_be_equal("abcd".to_string(), "cdab".to_string())
        );
        assert_eq!(
            false,
            Solution::can_be_equal("abcd".to_string(), "dacb".to_string())
        );
        assert_eq!(
            false,
            Solution::can_be_equal("cmpr".to_string(), "rmcp".to_string())
        );
        assert_eq!(
            false,
            Solution::can_be_equal("jegy".to_string(), "jeyg".to_string())
        );
    }
}
