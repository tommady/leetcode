// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
//
//
// "abc"
// "pqr"
// "ab"
// "pqrs"
// "abcd"
// "pq"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let total = word1.len() + word2.len();
        let mut ret = String::with_capacity(total);

        let mut world1_iter = word1.chars();
        let mut world2_iter = word2.chars();

        while ret.len() < total {
            if let Some(c) = world1_iter.next() {
                ret.push(c);
            }
            if let Some(c) = world2_iter.next() {
                ret.push(c);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1768_solution() {
        assert_eq!(
            "apbqcr".to_string(),
            Solution::merge_alternately("abc".to_string(), "pqr".to_string())
        );
        assert_eq!(
            "apbqrs".to_string(),
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string())
        );
        assert_eq!(
            "apbqcd".to_string(),
            Solution::merge_alternately("abcd".to_string(), "pq".to_string())
        );
    }
}