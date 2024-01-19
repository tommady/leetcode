// Given an array words of strings made only from lowercase letters, return a list of all characters that show up in all strings within the list (including duplicates). For example, if a character occurs 3 times in all strings but not 4 times, you need to include that character three times in the final answer. You may return the answer in any order. Example 1: Input: [ bella , label , roller ] Output: [ e , l , l ] Example 2: Input: [ cool , lock , cook ] Output: [ c , o ] Note: 1 = words.length = 100 1 = words[i].length = 100 words[i] consists of lowercase English letters.
//
//
// ["bella","label","roller"]
// ["cool","lock","cook"]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut ret = Vec::with_capacity(words.len());
        let mut cache: [[usize; 26]; 100] = [[0; 26]; 100];

        for (i, word) in words.iter().enumerate() {
            for b in word.as_bytes() {
                cache[i][*b as usize - 97] += 1;
            }
        }

        fn min(a: usize, b: usize) -> usize {
            if a > b {
                return b;
            }
            a
        }

        for i in 0..26 {
            if cache[0][i] != 0 {
                let mut is_ok = true;
                let mut min_counter = cache[0][i];

                for c in cache.iter().take(words.len()).skip(1) {
                    if c[i] == 0 {
                        is_ok = false;
                        min_counter = 0;
                        break;
                    }

                    min_counter = min(min_counter, c[i])
                }
                if is_ok {
                    for _ in 0..min_counter {
                        ret.push(String::from_utf8_lossy(&[i as u8 + 97]).into())
                    }
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1002_solution() {
        // You may return the answer in any order.

        assert_eq!(
            vec!["e".to_string(), "l".to_string(), "l".to_string()],
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ])
        );
        assert_eq!(
            vec!["c".to_string(), "o".to_string()],
            Solution::common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ])
        );
        assert_eq!(
            Vec::<String>::new(),
            Solution::common_chars(vec![
                "acabcddd".to_string(),
                "bcbdbcbd".to_string(),
                "baddbadb".to_string(),
                "cbdddcac".to_string(),
                "aacbcccd".to_string(),
                "ccccddda".to_string(),
                "cababaab".to_string(),
                "addcaccd".to_string()
            ])
        )
    }
}
