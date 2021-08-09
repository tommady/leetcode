// A string is good if there are no repeated characters. Given a string s​​​​​, return the number of good substrings of length three in s​​​​​​. Note that if there are multiple occurrences of the same substring, every occurrence should be counted. A substring is a contiguous sequence of characters in a string. Example 1: Input: s = xyzzaz Output: 1 Explanation: There are 4 substrings of size 3: xyz , yzz , zza , and zaz . The only good substring of length 3 is xyz . Example 2: Input: s = aababcabc Output: 4 Explanation: There are 7 substrings of size 3: aab , aba , bab , abc , bca , cab , and abc . The good substrings are abc , bca , cab , and abc . Constraints: 1 = s.length = 100 s​​​​​​ consists of lowercase English letters.
//
//
// "xyzzaz"
// "aababcabc"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let n = s.len();

        if n < 3 {
            return 0;
        }
        let mut ret = 0;
        let bytes = s.into_bytes();

        for i in 0..n - 2 {
            let a = bytes[i];
            let b = bytes[i + 1];
            let c = bytes[i + 2];
            if a != b && b != c && a != c {
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
    fn test_1876_solution() {
        assert_eq!(1, Solution::count_good_substrings("xyzzaz".to_string()));
        assert_eq!(4, Solution::count_good_substrings("aababcabc".to_string()));
    }
}
