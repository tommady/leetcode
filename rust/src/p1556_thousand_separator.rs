// Given an integer n, add a dot ( . ) as the thousands separator and return it in string format. Example 1: Input: n = 987 Output: 987 Example 2: Input: n = 1234 Output: 1.234 Example 3: Input: n = 123456789 Output: 123.456.789 Example 4: Input: n = 0 Output: 0 Constraints: 0 = n 2^31
//
//
// 987
// 1234
// 123456789
// 0
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n < 1000 {
            return n.to_string();
        }

        let mut ret: Vec<char> = n.to_string().chars().collect();
        ret.reverse();
        let mut i = 3;
        while i < ret.len() {
            ret.insert(i, '.');
            i += 4;
        }

        ret.reverse();
        ret.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1556_solution() {
        assert_eq!("987".to_string(), Solution::thousand_separator(987));
        assert_eq!("1.234".to_string(), Solution::thousand_separator(1234));
        assert_eq!(
            "123.456.789".to_string(),
            Solution::thousand_separator(123456789)
        );
        assert_eq!("0".to_string(), Solution::thousand_separator(0));
    }
}
