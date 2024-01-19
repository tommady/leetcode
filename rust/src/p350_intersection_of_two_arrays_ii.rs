// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order. Example 1: Input: nums1 = [1,2,2,1], nums2 = [2,2] Output: [2,2] Example 2: Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4] Output: [4,9] Explanation: [9,4] is also accepted. Constraints: 1 = nums1.length, nums2.length = 1000 0 = nums1[i], nums2[i] = 1000 Follow up: What if the given array is already sorted? How would you optimize your algorithm? What if nums1&#39;s size is small compared to nums2&#39;s size? Which algorithm is better? What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
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
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut big_one: &Vec<i32> = nums2.as_ref();
        let mut small_one: &Vec<i32> = nums1.as_ref();

        if nums1.len() > nums2.len() {
            big_one = nums1.as_ref();
            small_one = nums2.as_ref();
        }

        let mut cache = [0; 1000];

        for n in big_one.iter() {
            cache[*n as usize] += 1;
        }

        let mut ret = Vec::with_capacity(small_one.len());
        for n in small_one.iter() {
            if cache[*n as usize] != 0 {
                ret.push(*n);
                cache[*n as usize] -= 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_350_solution() {
        // you may return the result in any order

        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );
        assert_eq!(
            vec![4, 9],
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
