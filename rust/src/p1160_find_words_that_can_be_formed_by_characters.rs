// You are given an array of strings words and a string chars. A string is good if it can be formed by characters from chars (each character can only be used once). Return the sum of lengths of all good strings in words. Example 1: Input: words = [ cat , bt , hat , tree ], chars = atach Output: 6 Explanation: The strings that can be formed are cat and hat so the answer is 3 + 3 = 6. Example 2: Input: words = [ hello , world , leetcode ], chars = welldonehoneyr Output: 10 Explanation: The strings that can be formed are hello and world so the answer is 5 + 5 = 10. Constraints: 1 = words.length = 1000 1 = words[i].length, chars.length = 100 words[i] and chars consist of lowercase English letters.
//
//
// ["cat","bt","hat","tree"]
// "atach"
// ["hello","world","leetcode"]
// "welldonehoneyr"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut ret = 0;
        let mut m = vec![0; 26];

        for b in chars.as_bytes() {
            m[*b as usize - 97] += 1;
        }

        for word in words {
            let mut wm = vec![0; 26];
            for w in word.as_bytes() {
                wm[*w as usize - 97] += 1;
            }

            let mut temp = 0;
            let mut is_good = true;
            for i in 0..wm.len() {
                if wm[i] <= m[i] {
                    temp += wm[i];
                } else {
                    is_good = false;
                    break;
                }
            }

            if is_good {
                ret += temp;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1160_solution() {
        assert_eq!(
            6,
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            )
        );
        assert_eq!(
            10,
            Solution::count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            )
        );
    }
}
