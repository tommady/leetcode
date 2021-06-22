// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order. Example 1: Input: nums = [1,1,1,2,2,3], k = 2 Output: [1,2] Example 2: Input: nums = [1], k = 1 Output: [1] Constraints: 1 = nums.length = 105 k is in the range [1, the number of unique elements in the array]. It is guaranteed that the answer is unique. Follow up: Your algorithm&#39;s time complexity must be better than O(n log n), where n is the array&#39;s size.
//
//
// [1,1,1,2,2,3]
// 2
// [1]
// 1
//

#![allow(dead_code)]
use std::collections::{BinaryHeap, HashMap};
pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        let mut map = HashMap::with_capacity(nums.len());

        for n in nums.into_iter() {
            *map.entry(n).or_insert(0) += 1;
        }

        for (k, v) in map.into_iter() {
            heap.push((v, k));
        }

        let mut ret = Vec::with_capacity(k as usize);
        for _ in 0..k {
            ret.push(heap.pop().unwrap().1);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_347_solution() {
        assert_eq!(
            vec![1, 2],
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
        );
        assert_eq!(vec![1], Solution::top_k_frequent(vec![1], 1));
    }
}
