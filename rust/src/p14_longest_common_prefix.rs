// Write a function to find the longest common prefix string amongst an array of strings. If there is no common prefix, return an empty string . Example 1: Input: strs = [ flower , flow , flight ] Output: fl Example 2: Input: strs = [ dog , racecar , car ] Output: Explanation: There is no common prefix among the input strings. Constraints: 1 = strs.length = 200 0 = strs[i].length = 200 strs[i] consists of only lower-case English letters.
//
//
// ["flower","flow","flight"]
// ["dog","racecar","car"]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = String::from("");
        let mut i = 0;

        'lp: loop {
            let mut c = '?';
            for s in strs.iter() {
                if i >= s.len() {
                    break 'lp;
                }

                let cc = s.as_bytes()[i] as char;
                if c != cc && c != '?' {
                    break 'lp;
                }

                c = cc;
            }

            i += 1;
            ret.push(c);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14_solution() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "".to_string(),
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
    }
}
