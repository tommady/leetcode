// You are given a 0-indexed string s typed by a user. Changing a key is defined as using a key different from the last used key. For example, s = ab has a change of a key while s = bBBb does not have any. Return the number of times the user had to change the key. Note: Modifiers like shift or caps lock won&#39;t be counted in changing the key that is if a user typed the letter &#39;a&#39; and then the letter &#39;A&#39; then it will not be considered as a changing of key. Example 1: Input: s = aAbBcC Output: 2 Explanation: From s[0] = &#39;a&#39; to s[1] = &#39;A&#39;, there is no change of key as caps lock or shift is not counted. From s[1] = &#39;A&#39; to s[2] = &#39;b&#39;, there is a change of key. From s[2] = &#39;b&#39; to s[3] = &#39;B&#39;, there is no change of key as caps lock or shift is not counted. From s[3] = &#39;B&#39; to s[4] = &#39;c&#39;, there is a change of key. From s[4] = &#39;c&#39; to s[5] = &#39;C&#39;, there is no change of key as caps lock or shift is not counted. Example 2: Input: s = AaAaAaaA Output: 0 Explanation: There is no change of key since only the letters &#39;a&#39; and &#39;A&#39; are pressed which does not require change of key. Constraints: 1 = s.length = 100 s consists of only upper case and lower case English letters.
//
//
// "aAbBcC"
// "AaAaAaaA"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut ret = 0;
        let bytes_string = s.as_bytes();

        for i in 1..bytes_string.len() {
            if bytes_string[i - 1].to_ascii_lowercase() != bytes_string[i].to_ascii_lowercase() {
                ret += 1
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3019_solution() {
        assert_eq!(2, Solution::count_key_changes("aAbBcC".to_string()));
        assert_eq!(0, Solution::count_key_changes("AaAaAaaA".to_string()));
    }
}
