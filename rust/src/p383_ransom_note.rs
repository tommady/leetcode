// Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise. Each letter in magazine can only be used once in ransomNote. Example 1: Input: ransomNote = "a", magazine = "b" Output: false Example 2: Input: ransomNote = "aa", magazine = "ab" Output: false Example 3: Input: ransomNote = "aa", magazine = "aab" Output: true Constraints: 1 = ransomNote.length, magazine.length = 105 ransomNote and magazine consist of lowercase English letters.
//
//
// "a"
// "b"
// "aa"
// "ab"
// "aa"
// "aab"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for c in ransom_note.chars() {
            m.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for c in magazine.chars() {
            m.entry(c).and_modify(|counter| *counter -= 1);
        }

        for v in m.values() {
            if v.gt(&0) {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383_solution() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
        assert!(Solution::can_construct(
            "aab".to_string(),
            "baa".to_string()
        ));
    }
}
