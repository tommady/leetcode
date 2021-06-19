// Given an array of integers arr and an integer k. Find the least number of unique integers after removing exactly k elements.      Example 1:   Input: arr = [5,5,4], k = 1 Output: 1 Explanation: Remove the single 4, only 5 is left.  Example 2:   Input: arr = [4,3,1,1,3,3,2], k = 3 Output: 2 Explanation: Remove 4, 2 and either one of the two 1s or three 3s. 1 and 3 will be left.   Constraints:   1 = arr.length = 10^5 1 = arr[i] = 10^9 0 = k = arr.length
//
//
// [5,5,4]
// 1
// [4,3,1,1,3,3,2]
// 3
//

#![allow(dead_code)]

use std::collections::{BinaryHeap, HashMap};

pub struct Solution {}

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(arr.len());
        for a in arr {
            *map.entry(a).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(map.len());
        for v in map.values() {
            heap.push(*v);
        }

        let mut ret: i32 = heap.len() as i32;
        for v in heap.into_sorted_vec() {
            if v - k <= 0 {
                ret -= 1;
            }

            k -= v;
            if k <= 0 {
                break;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1481_solution() {
        assert_eq!(1, Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1));
        assert_eq!(
            2,
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3)
        );
        assert_eq!(
            1,
            Solution::find_least_num_of_unique_ints(vec![2, 1, 1, 3, 3, 3], 3)
        );
    }
}
