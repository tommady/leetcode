// You are given a non-negative integer array nums. In one operation, you must: Choose a positive integer x such that x is less than or equal to the smallest non-zero element in nums. Subtract x from every positive element in nums. Return the minimum number of operations to make every element in nums equal to 0. Example 1: Input: nums = [1,5,0,3,5] Output: 3 Explanation: In the first operation, choose x = 1. Now, nums = [0,4,0,2,4]. In the second operation, choose x = 2. Now, nums = [0,2,0,0,2]. In the third operation, choose x = 2. Now, nums = [0,0,0,0,0]. Example 2: Input: nums = [0] Output: 0 Explanation: Each element in nums is already 0 so no operations are needed. Constraints: 1 = nums.length = 100 0 = nums[i] = 100
//
//
// [1,5,0,3,5]
// [0]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut nums = nums;
        let mut sub = 0;
        nums.sort();

        for num in nums {
            if num - sub <= 0 {
                continue;
            }
            sub += num - sub;
            ret += 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2357_solution() {
        assert_eq!(3, Solution::minimum_operations(vec![1, 5, 0, 3, 5]));
        assert_eq!(0, Solution::minimum_operations(vec![0]));
    }
}
