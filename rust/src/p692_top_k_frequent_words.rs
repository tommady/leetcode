// Given a non-empty list of words, return the k most frequent elements. Your answer should be sorted by frequency from highest to lowest. If two words have the same frequency, then the word with the lower alphabetical order comes first.  Example 1:  Input: ["i", "love", "leetcode", "i", "love", "coding"], k = 2 Output: ["i", "love"] Explanation: "i" and "love" are the two most frequent words. Note that "i" comes before "love" due to a lower alphabetical order.    Example 2:  Input: ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4 Output: ["the", "is", "sunny", "day"] Explanation: "the", "is", "sunny" and "day" are the four most frequent words, with the number of occurrence being 4, 3, 2 and 1 respectively.    Note:  You may assume k is always valid, 1 k number of unique elements. Input words contain only lowercase letters.    Follow up:  Try to solve it in O(n log k) time and O(n) extra space.
//
//
// ["i", "love", "leetcode", "i", "love", "coding"]
// 2
//

#![allow(dead_code)]
use std::collections::{BinaryHeap, HashMap};
pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        let mut map = HashMap::with_capacity(words.len());

        for word in words.iter() {
            *map.entry(word).or_insert(0) += 1;
        }

        #[derive(Eq, PartialEq, Debug)]
        struct Record {
            count: usize,
            word: String,
        }
        impl PartialOrd for Record {
            fn partial_cmp(&self, other: &Record) -> Option<std::cmp::Ordering> {
                if self.count == other.count {
                    return Some(other.word.cmp(&self.word));
                }
                Some(self.count.cmp(&other.count))
            }
        }
        impl Ord for Record {
            fn cmp(&self, other: &Record) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        for (k, v) in map.into_iter() {
            heap.push(Record {
                count: v,
                word: k.to_string(),
            });
        }

        let mut ret = Vec::with_capacity(k as usize);
        for _ in 0..k {
            ret.push(heap.pop().unwrap().word);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_692_solution() {
        assert_eq!(
            vec!["i".to_string(), "love".to_string()],
            Solution::top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2
            )
        )
    }
}
