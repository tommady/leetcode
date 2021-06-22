// Given a non-empty list of words, return the k most frequent elements.
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