// We have a collection of stones, each stone has a positive integer weight. Each turn, we choose the two heaviest stones and smash them together. Suppose the stones have weights x and y with x = y. The result of this smash is: If x == y, both stones are totally destroyed; If x != y, the stone of weight x is totally destroyed, and the stone of weight y has new weight y-x. At the end, there is at most 1 stone left. Return the weight of this stone (or 0 if there are no stones left.) Example 1: Input: [2,7,4,1,8,1] Output: 1 Explanation: We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then, we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then, we combine 2 and 1 to get 1 so the array converts to [1,1,1] then, we combine 1 and 1 to get 0 so the array converts to [1] then that&#39;s the value of last stone. Note: 1 = stones.length = 30 1 = stones[i] = 1000
//
//
// [2,7,4,1,8,1]
//

#![allow(dead_code)]
use std::collections::BinaryHeap;
pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<Box<i32>> = BinaryHeap::with_capacity(stones.len());

        for stone in stones.into_iter() {
            heap.push(Box::new(stone));
        }

        fn abs(a: i32) -> i32 {
            if a < 0 {
                return -a;
            }
            a
        }

        loop {
            let a = *heap.pop().unwrap_or(Box::new(0));
            let b = *heap.pop().unwrap_or(Box::new(0));

            let c = abs(a - b);
            if c != 0 {
                heap.push(Box::new(c));
            }

            if heap.len() <= 1 {
                break;
            }
        }

        if heap.len() == 0 {
            return 0;
        }

        *heap.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1046_solution() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
        assert_eq!(1, Solution::last_stone_weight(vec![1]));
    }
}
