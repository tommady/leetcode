// Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise. Example 1: Input: arr = [1,2,2,1,1,3] Output: true Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences. Example 2: Input: arr = [1,2] Output: false Example 3: Input: arr = [-3,0,1,-3,1,1,1,-3,10,0] Output: true Constraints: 1 = arr.length = 1000 -1000 = arr[i] = 1000
//
//
// [1,2,2,1,1,3]
// [1,2]
// [-3,0,1,-3,1,1,1,-3,10,0]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let mut map = HashMap::with_capacity(arr.len());
        for a in &arr {
            let _ = *map.entry(a).and_modify(|e| *e += 1).or_insert(0);
        }

        let mut check = HashMap::with_capacity(map.len());
        for (k, v) in map.into_iter() {
            if let Some(e) = check.get(&v) {
                if e != k {
                    return false;
                }
            } else {
                check.insert(v, *k);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1207_solution() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert!(!Solution::unique_occurrences(vec![1, 2]));
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }
}
