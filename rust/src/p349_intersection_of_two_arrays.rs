// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order. Example 1: Input: nums1 = [1,2,2,1], nums2 = [2,2] Output: [2] Example 2: Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4] Output: [9,4] Explanation: [4,9] is also accepted. Constraints: 1 = nums1.length, nums2.length = 1000 0 = nums1[i], nums2[i] = 1000
//
//
// [1,2,2,1]
// [2,2]
// [4,9,5]
// [9,4,9,8,4]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut big_one: &Vec<i32> = nums1.as_ref();
        let mut small_one: &Vec<i32> = nums2.as_ref();

        if nums2.len() > nums1.len() {
            big_one = nums2.as_ref();
            small_one = nums1.as_ref();
        }

        let mut cache = [0; 1000];
        for n in big_one.iter() {
            cache[*n as usize] += 1;
        }

        let mut ret = Vec::with_capacity(small_one.len());
        for n in small_one.iter() {
            if cache[*n as usize] != 0 {
                ret.push(*n);
                cache[*n as usize] = 0;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_349_solution() {
        // you may return the result in any order

        assert_eq!(
            vec![2].sort(),
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]).sort()
        );
        assert_eq!(
            vec![9, 4].sort(),
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]).sort()
        );
    }
}
