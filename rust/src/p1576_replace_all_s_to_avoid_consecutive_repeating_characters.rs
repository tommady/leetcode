// Given a string s containing only lowercase English letters and the &#39;?&#39; character, convert all the &#39;?&#39; characters into lowercase letters such that the final string does not contain any consecutive repeating characters. You cannot modify the non &#39;?&#39; characters. It is guaranteed that there are no consecutive repeating characters in the given string except for &#39;?&#39;. Return the final string after all the conversions (possibly zero) have been made. If there is more than one solution, return any of them. It can be shown that an answer is always possible with the given constraints. Example 1: Input: s = ?zs Output: azs Explanation: There are 25 solutions for this problem. From azs to yzs , all are valid. Only z is an invalid modification as the string will consist of consecutive repeating characters in zzs . Example 2: Input: s = ubv?w Output: ubvaw Explanation: There are 24 solutions for this problem. Only v and w are invalid modifications as the strings will consist of consecutive repeating characters in ubvvw and ubvww . Example 3: Input: s = j?qg??b Output: jaqgacb Example 4: Input: s = ??yw?ipkj? Output: acywaipkja Constraints: 1 = s.length = 100 s consist of lowercase English letters and &#39;?&#39;.
//
//
// "?zs"
// "ubv?w"
// "j?qg??b"
// "??yw?ipkj?"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut ret = s.clone().into_bytes();

        for i in 0..ret.len() {
            let mut m = [true; 26];
            if ret[i] == b'?' {
                let mut prev: u8 = 0;
                let mut next: u8 = b'?';
                if i + 1 < ret.len() {
                    next = ret[i + 1];
                }
                if (i as i32 - 1) >= 0 {
                    prev = ret[i - 1];
                }

                if prev != 0 {
                    m[prev as usize - 97] = false;
                }
                if next != b'?' {
                    m[next as usize - 97] = false;
                }

                for (j, n) in m.iter().enumerate() {
                    if *n {
                        ret[i] = j as u8 + 97;
                        break;
                    }
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1576_solution() {
        assert_eq!(
            "azs".to_string(),
            Solution::modify_string("?zs".to_string())
        );
        assert_eq!(
            "ubvaw".to_string(),
            Solution::modify_string("ubv?w".to_string())
        );
        assert_eq!(
            "jaqgacb".to_string(),
            Solution::modify_string("j?qg??b".to_string())
        );
        assert_eq!(
            "abywaipkja".to_string(),
            Solution::modify_string("??yw?ipkj?".to_string())
        );
    }
}
