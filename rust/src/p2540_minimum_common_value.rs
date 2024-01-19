// Given two integer arrays nums1 and nums2, sorted in non-decreasing order, return the minimum integer common to both arrays. If there is no common integer amongst nums1 and nums2, return -1. Note that an integer is said to be common to nums1 and nums2 if both arrays have at least one occurrence of that integer. Example 1: Input: nums1 = [1,2,3], nums2 = [2,4] Output: 2 Explanation: The smallest element common to both arrays is 2, so we return 2. Example 2: Input: nums1 = [1,2,3,6], nums2 = [2,3,4,5] Output: 2 Explanation: There are two common elements in the array 2 and 3 out of which 2 is the smallest, so 2 is returned. Constraints: 1 = nums1.length, nums2.length = 105 1 = nums1[i], nums2[j] = 109 Both nums1 and nums2 are sorted in non-decreasing order.
//
//
// [1,2,3]
// [2,4]
// [1,2,3,6]
// [2,3,4,5]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut ret = i32::MAX;

        for n in nums1 {
            if map.get(&n).is_none() {
                map.insert(n, 1);
            }
        }

        for n in nums2 {
            if let Some(v) = map.get_mut(&n) {
                *v += 1;
            }
        }

        for (k, v) in map {
            if v != 1 && k < ret {
                ret = k
            }
        }

        if ret.eq(&i32::MAX) {
            return -1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2540_solution() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3], vec![2, 4]));
        assert_eq!(2, Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]));
        assert_eq!(2, Solution::get_common(vec![1, 1, 2], vec![2, 4]));
    }
}
