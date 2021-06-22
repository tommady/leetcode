// Given a string s, sort it in decreasing order based on the frequency of characters, and return the sorted string. Example 1: Input: s = tree Output: eert Explanation: &#39;e&#39; appears twice while &#39;r&#39; and &#39;t&#39; both appear once. So &#39;e&#39; must appear before both &#39;r&#39; and &#39;t&#39;. Therefore eetr is also a valid answer. Example 2: Input: s = cccaaa Output: aaaccc Explanation: Both &#39;c&#39; and &#39;a&#39; appear three times, so aaaccc is also a valid answer. Note that cacaca is incorrect, as the same characters must be together. Example 3: Input: s = Aabb Output: bbAa Explanation: bbaA is also a valid answer, but Aabb is incorrect. Note that &#39;A&#39; and &#39;a&#39; are treated as two different characters. Constraints: 1 = s.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 105 s consists of English letters and digits.
//
//
// "tree"
// "cccaaa"
// "Aabb"
//

#![allow(dead_code)]
use std::collections::{BinaryHeap, HashMap};
pub struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut heap = BinaryHeap::with_capacity(s.len());
        let mut map = HashMap::with_capacity(s.len());

        for b in s.as_bytes() {
            *map.entry(b).or_insert(0) += 1;
        }

        #[derive(Eq, PartialEq)]
        struct Record {
            count: usize,
            b: u8,
        }

        impl PartialOrd for Record {
            fn partial_cmp(&self, other: &Record) -> Option<std::cmp::Ordering> {
                if self.count == other.count {
                    return Some(other.b.cmp(&self.b));
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
            heap.push(Record { count: v, b: *k });
        }

        let mut ret = Vec::with_capacity(s.len());
        for _ in 0..heap.len() {
            let r = heap.pop().unwrap();
            for _ in 0..r.count {
                ret.push(r.b);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_451_solution() {
        assert_eq!(
            "eert".to_string(),
            Solution::frequency_sort("tree".to_string())
        );
        assert_eq!(
            "aaaccc".to_string(),
            Solution::frequency_sort("cccaaa".to_string())
        );
        assert_eq!(
            "bbAa".to_string(),
            Solution::frequency_sort("Aabb".to_string())
        );
    }
}
