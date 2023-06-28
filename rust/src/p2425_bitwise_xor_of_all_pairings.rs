// You are given two 0-indexed arrays, nums1 and nums2, consisting of non-negative integers. There exists another array, nums3, which contains the bitwise XOR of all pairings of integers between nums1 and nums2 (every integer in nums1 is paired with every integer in nums2 exactly once). Return the bitwise XOR of all integers in nums3. Example 1: Input: nums1 = [2,1,3], nums2 = [10,2,5,0] Output: 13 Explanation: A possible nums3 array is [8,0,7,2,11,3,4,1,9,1,6,3]. The bitwise XOR of all these numbers is 13, so we return 13. Example 2: Input: nums1 = [1,2], nums2 = [3,4] Output: 0 Explanation: All possible pairs of bitwise XORs are nums1[0] ^ nums2[0], nums1[0] ^ nums2[1], nums1[1] ^ nums2[0], and nums1[1] ^ nums2[1]. Thus, one possible nums3 array is [2,5,1,6]. 2 ^ 5 ^ 1 ^ 6 = 0, so we return 0. Constraints: 1 = nums1.length, nums2.length = 105 0 = nums1[i], nums2[j] = 109
//
//
// [2,1,3]
// [10,2,5,0]
// [1,2]
// [3,4]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn xor(nums: &[i32]) -> i32 {
            nums.into_iter().fold(0, |acc, e| acc ^ e)
        }

        match (nums1.len() % 2, nums2.len() % 2) {
            (0, 0) => 0,
            (0, 1) => xor(&nums1),
            (1, 0) => xor(&nums2),
            _ => xor(&nums1) ^ xor(&nums2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2425_solution() {
        assert_eq!(13, Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]));
        assert_eq!(0, Solution::xor_all_nums(vec![1, 2], vec![3, 4]));
    }
}
