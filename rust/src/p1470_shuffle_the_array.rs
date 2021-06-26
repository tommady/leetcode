// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].  Return the array in the form [x1,y1,x2,y2,...,xn,yn].   Example 1:   Input: nums = [2,5,1,3,4,7], n = 3 Output: [2,3,5,4,1,7]  Explanation: Since x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 then the answer is [2,3,5,4,1,7].   Example 2:   Input: nums = [1,2,3,4,4,3,2,1], n = 4 Output: [1,4,2,3,3,2,4,1]   Example 3:   Input: nums = [1,1,2,2], n = 2 Output: [1,2,1,2]    Constraints:   1 = n = 500 nums.length == 2n 1 = nums[i] = 10^3
//
//
// [2,5,1,3,4,7]
// 3
// [1,2,3,4,4,3,2,1]
// 4
// [1,1,2,2]
// 2
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut x = 0 as usize;
        let mut y = n as usize;
        let mut ret = Vec::with_capacity(nums.len());

        while x <= n as usize - 1 && y < nums.len() {
            ret.push(nums[x]);
            ret.push(nums[y]);

            x += 1;
            y += 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1470_solution() {
        assert_eq!(
            vec![2, 3, 5, 4, 1, 7],
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3)
        );
        assert_eq!(
            vec![1, 4, 2, 3, 3, 2, 4, 1],
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4)
        );
        assert_eq!(vec![1, 2, 1, 2], Solution::shuffle(vec![1, 1, 2, 2], 2));
    }
}
