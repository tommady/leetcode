// Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order. Return the sorted array. Example 1: Input: nums = [1,1,2,2,2,3] Output: [3,1,1,2,2,2] Explanation: &#39;3&#39; has a frequency of 1, &#39;1&#39; has a frequency of 2, and &#39;2&#39; has a frequency of 3. Example 2: Input: nums = [2,3,1,3,2] Output: [1,3,3,2,2] Explanation: &#39;2&#39; and &#39;3&#39; both have a frequency of 2, so they are sorted in decreasing order. Example 3: Input: nums = [-1,1,-6,4,5,-6,1,4,1] Output: [5,-1,4,4,-6,-6,1,1,1] Constraints: 1 = nums.length = 100 -100 = nums[i] = 100
//
//
// [1,1,2,2,2,3]
// [2,3,1,3,2]
// [-1,1,-6,4,5,-6,1,4,1]
//

#![allow(dead_code)]
use std::collections::{BinaryHeap, HashMap};
pub struct Solution {}

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        let mut map = HashMap::with_capacity(nums.len());

        for n in nums.iter() {
            *map.entry(n).or_insert(0) += 1;
        }

        #[derive(Eq, PartialEq)]
        struct Record {
            count: usize,
            n: i32,
        }

        impl PartialOrd for Record {
            fn partial_cmp(&self, other: &Record) -> Option<std::cmp::Ordering> {
                if self.count == other.count {
                    return Some(self.n.cmp(&other.n));
                }
                Some(other.count.cmp(&self.count))
            }
        }

        impl Ord for Record {
            fn cmp(&self, other: &Record) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        for (k, v) in map.into_iter() {
            heap.push(Record { count: v, n: *k });
        }

        let mut ret = Vec::with_capacity(nums.len());
        for _ in 0..heap.len() {
            let r = heap.pop().unwrap();
            for _ in 0..r.count {
                ret.push(r.n);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1636_solution() {
        assert_eq!(
            vec![3, 1, 1, 2, 2, 2],
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3])
        );
        assert_eq!(
            vec![1, 3, 3, 2, 2],
            Solution::frequency_sort(vec![2, 3, 1, 3, 2])
        );
        assert_eq!(
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])
        );
    }
}
