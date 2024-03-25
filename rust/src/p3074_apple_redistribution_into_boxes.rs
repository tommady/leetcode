// You are given an array apple of size n and an array capacity of size m.
// There are n packs where the ith pack contains apple[i] apples. There are m boxes as well, and the ith box has a capacity of capacity[i] apples.
// Return the
// minimum number of boxes you need to select to redistribute these n packs of apples into boxes.
//
// Note that, apples from the same pack can be distributed into different boxes.
//
// Example 1:
//
// Input: apple = [1,3,2], capacity = [4,3,1,5,2]
// Output: 2
// Explanation: We will use boxes with capacities 4 and 5. It is possible to distribute the apples as the total capacity is greater than or equal to the total number of apples.  Example 2:
//
// Input: apple = [5,5,5], capacity = [2,4,2,7]
// Output: 4
// Explanation: We will need to use all the boxes.
//
// Constraints:
//  1 = n == apple.length = 50 1 = m == capacity.length = 50 1 = apple[i], capacity[i] = 50 The input is generated such that it's possible to redistribute packs of apples into boxes.

// hint 1
// Sort array capacity in non-decreasing order.

// hint 2
// Select boxes greedily while the total capacity is smaller than the total number of apples.

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let total_apples = apple.into_iter().sum();
        let mut capacity = capacity;
        capacity.sort();
        capacity.reverse();

        if capacity[0] >= total_apples {
            return 1;
        }

        for i in 1..capacity.len() {
            capacity[i] += capacity[i - 1];
            if capacity[i] >= total_apples {
                return (i + 1) as i32;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_3074_solution() {
        assert_eq!(
            2,
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2])
        );
        assert_eq!(4, Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]));
    }
}
