// You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.
//
// Return the number of consistent strings in the array words.
//
//
// Example 1:
//
//
// Input: allowed = ab , words = [ ad , bd , aaab , baa , badab ]
// Output: 2
// Explanation: Strings aaab and baa are consistent since they only contain characters &#39;a&#39; and &#39;b&#39;.
//
//
// Example 2:
//
//
// Input: allowed = abc , words = [ a , b , c , ab , ac , bc , abc ]
// Output: 7
// Explanation: All strings are consistent.
//
//
// Example 3:
//
//
// Input: allowed = cad , words = [ cc , acd , b , ba , bac , bad , ac , d ]
// Output: 4
// Explanation: Strings cc , acd , ac , and d are consistent.
//
//
//
// Constraints:
//
//
// 	1 = words.length = 104
// 	1 = allowed.length = 26
// 	1 = words[i].length = 10
// 	The characters in allowed are distinct.
// 	words[i] and allowed contain only lowercase English letters.
//
//
// "ab"
// ["ad","bd","aaab","baa","badab"]
// "abc"
// ["a","b","c","ab","ac","bc","abc"]
// "cad"
// ["cc","acd","b","ba","bac","bad","ac","d"]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut m = [0; 123];
        for a in allowed.as_bytes() {
            m[*a as usize] = 1;
        }

        let mut counter = 0;
        for word in words {
            let mut found = true;
            for b in word.as_bytes() {
                if m[*b as usize] != 1 {
                    found = false;
                    break;
                }
            }
            if found {
                counter += 1;
            }
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1684_solution() {
        assert_eq!(
            2,
            Solution::count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string(),
                ]
            )
        );
        assert_eq!(
            7,
            Solution::count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string(),
                ]
            )
        );
        assert_eq!(
            4,
            Solution::count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string(),
                ]
            )
        );
    }
}
