// Given a string s and an integer array indices of the same length.
//
//
// "codeleet"
// [4,5,6,7,0,2,1,3]
// "abc"
// [0,1,2]
// "aiohn"
// [3,1,4,2,0]
// "aaiougrt"
// [4,0,2,6,7,3,1,5]
// "art"
// [1,0,2]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ret: Vec<u8> = vec![0; indices.len()];

        for i in 0..indices.len() {
            ret[indices[i] as usize] = s.as_bytes()[i];
        }

        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1528_solution() {
        assert_eq!(
            "leetcode".to_string(),
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3])
        );
        assert_eq!(
            "abc".to_string(),
            Solution::restore_string("abc".to_string(), vec![0, 1, 2])
        );
        assert_eq!(
            "nihao".to_string(),
            Solution::restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0])
        );
        assert_eq!(
            "arigatou".to_string(),
            Solution::restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5])
        );
        assert_eq!(
            "rat".to_string(),
            Solution::restore_string("art".to_string(), vec![1, 0, 2])
        );
    }
}