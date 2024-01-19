// A valid encoding of an array of words is any reference string s and array of indices indices such that: words.length == indices.length The reference string s ends with the &#39;#&#39; character. For each index indices[i], the substring of s starting from indices[i] and up to (but not including) the next &#39;#&#39; character is equal to words[i]. Given an array of words, return the length of the shortest reference string s possible of any valid encoding of words. Example 1: Input: words = [ time , me , bell ] Output: 10 Explanation: A valid encoding would be s = time#bell# and indices = [0, 2, 5]. words[0] = time , the substring of s starting from indices[0] = 0 to the next &#39;#&#39; is underlined in time#bell# words[1] = me , the substring of s starting from indices[1] = 2 to the next &#39;#&#39; is underlined in time#bell# words[2] = bell , the substring of s starting from indices[2] = 5 to the next &#39;#&#39; is underlined in time#bell# Example 2: Input: words = [ t ] Output: 2 Explanation: A valid encoding would be s = t# and indices = [0]. Constraints: 1 = words.length = 2000 1 = words[i].length = 7 words[i] consists of only lowercase letters.
//
//
// ["time","me","bell"]
// ["t"]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|b| std::cmp::Reverse(b.len()));
        let mut ret: i32 = 0;

        for word in words.iter() {
            ret += word.len() as i32 + 1;
        }

        let mut tmp = String::new();
        for word in words.iter() {
            let w = &format!("{}#", word);
            if tmp.contains(w) {
                ret -= word.len() as i32 + 1;
            }
            tmp += w;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_820_solution() {
        assert_eq!(
            10,
            Solution::minimum_length_encoding(vec![
                "time".to_string(),
                "me".to_string(),
                "bell".to_string()
            ])
        );
        assert_eq!(2, Solution::minimum_length_encoding(vec!["t".to_string()]));
        assert_eq!(
            5,
            Solution::minimum_length_encoding(vec!["me".to_string(), "time".to_string()])
        );
        assert_eq!(
            10,
            Solution::minimum_length_encoding(vec!["feipyxx".to_string(), "e".to_string()])
        );
    }
}
