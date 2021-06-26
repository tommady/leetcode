// You are given an integer array values where values[i] represents the value of the ith sightseeing spot. Two sightseeing spots i and j have a distance j - i between them. The score of a pair (i j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them. Return the maximum score of a pair of sightseeing spots. Example 1: Input: values = [8,1,5,2,6] Output: 11 Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11 Example 2: Input: values = [1,2] Output: 2 Constraints: 2 = values.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 104 1 = values[i] = 1000
//
//
// [8,1,5,2,6]
// [1,2]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut biggest: i32 = 0;
        let mut i = 0;

        for j in 1..values.len() {
            // the formular
            // 1. (i < j)
            // 2. values[i] + values[j] + i - j
            let result: i32 = values[i] + values[j] + i as i32 - j as i32;
            if result > biggest {
                biggest = result;
            }

            // looping the j and finding the possible i
            if values[j] + j as i32 > values[i] + i as i32 {
                i = j;
            }
        }

        biggest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1014_solution() {
        assert_eq!(
            11,
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6])
        );
        assert_eq!(2, Solution::max_score_sightseeing_pair(vec![1, 2]));
    }
}
