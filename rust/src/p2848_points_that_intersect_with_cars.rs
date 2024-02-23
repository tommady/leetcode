// You are given a
// 0-indexed 2D integer array nums representing the coordinates of the cars parking on a number line. For any index i, nums[i] = [starti, endi] where starti is the starting point of the ith car and endi is the ending point of the ith car.
// Return the number of integer points on the line that are covered with
// any part of a car.
//
// Example 1:
//
// Input: nums = [[3,6],[1,5],[4,7]]
// Output: 7
// Explanation: All the points from 1 to 7 intersect at least one car, therefore the answer would be 7.  Example 2:
//
// Input: nums = [[1,3],[5,8]]
// Output: 7
// Explanation: Points intersecting at least one car are 1, 2, 3, 5, 6, 7, 8. There are a total of 7 points, therefore the answer would be 7.
//
// Constraints:
//  1 = nums.length = 100 nums[i].length == 2 1 = starti = endi = 100

// hint 1
// Sort the array according to first element and then starting from the 0th index remove the overlapping parts and return the count of non-overlapping points.

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        let mut ret = ((nums[0][0]..nums[0][1]).count() + 1) as i32;

        for i in 1..nums.len() {
            if nums[i][0] < nums[i - 1][0] {
                ret += (nums[i][0]..nums[i - 1][0]).count() as i32;
            }

            if nums[i][0] > nums[i - 1][1] {
                ret += ((nums[i][0]..nums[i][1]).count() + 1) as i32
            } else if nums[i][1] > nums[i - 1][1] {
                ret += (nums[i - 1][1]..nums[i][1]).count() as i32;
            }

            // adjust
            if nums[i][0] > nums[i - 1][0] {
                nums[i][0] = nums[i - 1][0]
            }
            if nums[i][1] < nums[i - 1][1] {
                nums[i][1] = nums[i - 1][1]
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2848_solution() {
        assert_eq!(
            7,
            Solution::number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]])
        );
        assert_eq!(7, Solution::number_of_points(vec![vec![1, 3], vec![5, 8]]));
        assert_eq!(
            8,
            Solution::number_of_points(vec![vec![4, 4], vec![9, 10], vec![9, 10], vec![3, 8]])
        );
    }
}
