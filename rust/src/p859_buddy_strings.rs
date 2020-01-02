// Given two strings A and B of lowercase letters,
// return true if and only if we can swap two letters in A so that the result equals B.
//
// Example 1:
//
// Input: A = "ab", B = "ba"
// Output: true
// Example 2:
//
// Input: A = "ab", B = "ab"
// Output: false
// Example 3:
//
// Input: A = "aa", B = "aa"
// Output: true
// Example 4:
//
// Input: A = "aaaaaaabc", B = "aaaaaaacb"
// Output: true
// Example 5:
//
// Input: A = "", B = "aa"
// Output: false
//
//
// Note:
//
// 0 <= A.length <= 20000
// 0 <= B.length <= 20000
// A and B consist only of lowercase letters.

#![allow(dead_code)]

use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut a_set = HashMap::<&str, usize>::new();
        let mut diff = Vec::new();

        for i in 0..a.len() {
            a_set.insert(&a[i..i + 1], i);

            if a[i..i + 1] != b[i..i + 1] {
                diff.push(i);
                if diff.len() > 2 {
                    return false;
                }
            }
        }

        if diff.len() == 0 {
            return a_set.len() != a.len();
        }

        diff.len() == 2
            && a[diff[0]..diff[0] + 1] == b[diff[1]..diff[1] + 1]
            && a[diff[1]..diff[1] + 1] == b[diff[0]..diff[0] + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_859_solution() {
        assert_eq!(
            true,
            Solution::buddy_strings("ab".to_owned(), "ba".to_owned()),
            "'ab' and 'ba'",
        );
        assert_eq!(
            false,
            Solution::buddy_strings("ab".to_owned(), "ab".to_owned()),
            "'ab' and 'ab'",
        );
        assert_eq!(
            true,
            Solution::buddy_strings("aa".to_owned(), "aa".to_owned()),
            "'aa' and 'aa'",
        );
        assert_eq!(
            true,
            Solution::buddy_strings("aaaaaaabc".to_owned(), "aaaaaaacb".to_owned()),
            "'aaaaaaabc' and 'aaaaaaacb'",
        );
        assert_eq!(
            false,
            Solution::buddy_strings("".to_owned(), "aa".to_owned()),
            "'' and 'aa'",
        );

        assert_eq!(
            false,
            Solution::buddy_strings("abcaa".to_owned(), "abcbb".to_owned()),
            "'abcaa' and 'abcbb'",
        );
        assert_eq!(
            false,
            Solution::buddy_strings("ab".to_owned(), "ca".to_owned()),
            "'ab' and 'ca'",
        );
        assert_eq!(
            true,
            Solution::buddy_strings("abab".to_owned(), "abab".to_owned()),
            "'abab' and 'abab'",
        );
    }
}
