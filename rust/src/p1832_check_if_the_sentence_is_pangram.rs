// A pangram is a sentence where every letter of the English alphabet appears at least once. Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise. Example 1: Input: sentence = thequickbrownfoxjumpsoverthelazydog Output: true Explanation: sentence contains at least one of every letter of the English alphabet. Example 2: Input: sentence = leetcode Output: false Constraints: 1 = sentence.length = 1000 sentence consists of lowercase English letters.
//
//
// "thequickbrownfoxjumpsoverthelazydog"
// "leetcode"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut array: [u8; 123] = [0; 123];
        let mut counter = 0;

        for s in sentence.as_bytes() {
            let i = *s as usize;
            array[i] += 1;
            if array[i] == 1 {
                counter += 1;
            }
        }

        counter == 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1832_solution() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
        assert!(!Solution::check_if_pangram("leetcode".to_string()));
    }
}
