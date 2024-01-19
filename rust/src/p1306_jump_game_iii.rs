// Given an array of non-negative integers arr, you are initially positioned at start index of the array. When you are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach to any index with value 0. Notice that you can not jump outside of the array at any time. Example 1: Input: arr = [4,2,3,0,3,1,2], start = 5 Output: true Explanation: All possible ways to reach at index 3 with value 0 are: index 5 - index 4 - index 1 - index 3 index 5 - index 6 - index 4 - index 1 - index 3 Example 2: Input: arr = [4,2,3,0,3,1,2], start = 0 Output: true Explanation: One possible way to reach at index 3 with value 0 is: index 0 - index 4 - index 1 - index 3 Example 3: Input: arr = [3,0,2,1,2], start = 2 Output: false Explanation: There is no way to reach at index 1 with value 0. Constraints: 1 = arr.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 104 0 = arr[i] arr.length 0 = start arr.length
//
//
// [4,2,3,0,3,1,2]
// 5
// [4,2,3,0,3,1,2]
// 0
// [3,0,2,1,2]
// 2
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        let mut i = start as usize;
        let mut stack = vec![i];

        while !stack.is_empty() {
            i = stack.remove(0);
            if arr[i] == 0 {
                return true;
            }
            if arr[i] == -1 {
                continue;
            }

            let next = arr[i] as usize;
            arr[i] = -1;

            if (i + next < arr.len()) && (arr[i + next] != -1) {
                stack.push(i + next);
            }
            if (i as i32 - next as i32 >= 0) && (arr[i - next] != -1) {
                stack.push(i - next);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1306_solution() {
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
        assert!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
    }
}
