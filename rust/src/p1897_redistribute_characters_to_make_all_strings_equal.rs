// You are given an array of strings words (0-indexed). In one operation, pick two distinct indices i and j, where words[i] is a non-empty string, and move any character from words[i] to any position in words[j]. Return true if you can make every string in words equal using any number of operations, and false otherwise. Example 1: Input: words = [ abc , aabc , bc ] Output: true Explanation: Move the first &#39;a&#39; in words[1] to the front of words[2], to make words[1] = abc and words[2] = abc . All the strings are now equal to abc , so return true. Example 2: Input: words = [ ab , a ] Output: false Explanation: It is impossible to make all the strings equal using the operation. Constraints: 1 = words.length = 100 1 = words[i].length = 100 words[i] consists of lowercase English letters.
//
//
// ["abc","aabc","bc"]
// ["ab","a"]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut array: [usize; 26] = [0; 26];

        for word in words.iter() {
            for b in word.as_bytes() {
                array[*b as usize - 97] += 1;
            }
        }

        for v in array.iter() {
            if v % words.len() != 0 {
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
    fn test_1897_solution() {
        assert_eq!(
            true,
            Solution::make_equal(vec![
                "abc".to_string(),
                "aabc".to_string(),
                "bc".to_string()
            ])
        );
        assert_eq!(
            false,
            Solution::make_equal(vec!["ab".to_string(), "a".to_string()])
        );
        assert_eq!(
            false,
            Solution::make_equal(vec!["a".to_string(), "b".to_string()])
        );
        assert_eq!(
            true,
            Solution::make_equal(vec![
                "caaaaa".to_string(),
                "aaaaaaaaa".to_string(),
                "a".to_string(),
                "bbb".to_string(),
                "bbbbbbbbb".to_string(),
                "bbb".to_string(),
                "cc".to_string(),
                "cccccccccccc".to_string(),
                "ccccccc".to_string(),
                "ccccccc".to_string(),
                "cc".to_string(),
                "cccc".to_string(),
                "c".to_string(),
                "cccccccc".to_string(),
                "c".to_string(),
            ])
        );
    }
}
