// You are given a string array words and a
// binary array groups both of length n, where words[i] is associated with groups[i].
// Your task is to select the
// longest alternating subsequence from words. A subsequence of words is alternating if for any two consecutive strings in the sequence, their corresponding elements in the binary array groups differ. Essentially, you are to choose strings such that adjacent elements have non-matching corresponding bits in the groups array.
// Formally, you need to find the longest subsequence of an array of indices [0, 1, ..., n - 1] denoted as [i0, i1, ..., ik-1], such that groups[ij] != groups[ij+1] for each 0 = j k - 1 and then find the words corresponding to these indices.
// Return the selected subsequence. If there are multiple answers, return
// any of them.
//
// Note: The elements in words are distinct.
//
// Example 1:
//
// Input: words = ["e","a","b"], groups = [0,0,1]
//
// Output: ["e","b"]
//
// Explanation: A subsequence that can be selected is ["e","b"] because groups[0] != groups[2]. Another subsequence that can be selected is ["a","b"] because groups[1] != groups[2]. It can be demonstrated that the length of the longest subsequence of indices that satisfies the condition is 2.
//  Example 2:
//
// Input: words = ["a","b","c","d"], groups = [1,0,1,1]
//
// Output: ["a","b","c"]
//
// Explanation: A subsequence that can be selected is ["a","b","c"] because groups[0] != groups[1] and groups[1] != groups[2]. Another subsequence that can be selected is ["a","b","d"] because groups[0] != groups[1] and groups[1] != groups[3]. It can be shown that the length of the longest subsequence of indices that satisfies the condition is 3.
//
//
// Constraints:
//  1 = n == words.length == groups.length = 100 1 = words[i].length = 10 groups[i] is either 0 or 1. words consists of
// distinct strings. words[i] consists of lowercase English letters.

// hint 1
// This problem can be solved greedily.

// hint 2
// Begin by constructing the answer starting with the first number in groups.

// hint 3
// For each index i in the range [1, n - 1], add i to the answer if groups[i] != groups[i - 1].

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut words = words;
        let mut ret = Vec::with_capacity(words.len());
        ret.push(std::mem::take(&mut words[0]));

        for i in 1..groups.len() {
            if groups[i] != groups[i - 1] {
                ret.push(std::mem::take(&mut words[i]));
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_2900_solution() {
        assert_eq!(
            vec!["e".to_string(), "b".to_string()],
            Solution::get_longest_subsequence(
                vec!["e".to_string(), "a".to_string(), "b".to_string()],
                vec![0, 0, 1]
            )
        );
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            Solution::get_longest_subsequence(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ],
                vec![1, 0, 1, 1]
            )
        );
    }
}
