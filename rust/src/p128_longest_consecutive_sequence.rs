// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence. You must write an algorithm that runs in O(n) time. Example 1: Input: nums = [100,4,200,1,3,2] Output: 4 Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4. Example 2: Input: nums = [0,3,7,2,5,8,4,6,0,1] Output: 9 Constraints: 0 = nums.length = 105 -109 = nums[i] = 109
//
//
// [100,4,200,1,3,2]
// [0,3,7,2,5,8,4,6,0,1]
//

#![allow(dead_code)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut heap = BinaryHeap::with_capacity(nums.len());

        for n in nums.iter() {
            heap.push(Reverse(n));
        }

        fn max(a: i32, b: i32) -> i32 {
            if a > b {
                return a;
            }
            b
        }

        let mut max_counter: i32 = 1;
        let mut counter: i32 = 1;
        let mut min: i32 = 1;
        while let Some(Reverse(n)) = heap.pop() {
            if min + 1 == *n {
                counter += 1;
            } else if min != *n {
                counter = 1;
            }
            min = *n;
            max_counter = max(max_counter, counter);
        }

        max_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128_solution() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }
}
