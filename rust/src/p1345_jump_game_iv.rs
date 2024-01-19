// Given an array of integers arr, you are initially positioned at the first index of the array. In one step you can jump from index i to index: i + 1 where: i + 1 arr.length. i - 1 where: i - 1 = 0. j where: arr[i] == arr[j] and i != j. Return the minimum number of steps to reach the last index of the array. Notice that you can not jump outside of the array at any time. Example 1: Input: arr = [100,-23,-23,404,100,23,23,23,3,404] Output: 3 Explanation: You need three jumps from index 0 -- 4 -- 3 -- 9. Note that index 9 is the last index of the array. Example 2: Input: arr = [7] Output: 0 Explanation: Start index is the last index. You don&#39;t need to jump. Example 3: Input: arr = [7,6,9,6,9,6,9,7] Output: 1 Explanation: You can jump directly from index 0 to index 7 which is last index of the array. Example 4: Input: arr = [6,1,9] Output: 2 Example 5: Input: arr = [11,22,7,7,7,7,7,7,7,22,13] Output: 3 Constraints: 1 = arr.length = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 104 -108 = arr[i] = 108
//
//
// [100,-23,-23,404,100,23,23,23,3,404]
// [7]
// [7,6,9,6,9,6,9,7]
// [6,1,9]
// [11,22,7,7,7,7,7,7,7,22,13]
//

#![allow(dead_code)]
use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut visited = vec![false; arr.len()];

        for (i, a) in arr.iter().enumerate().skip(1) {
            let m = map.entry(*a).or_insert(vec![]);
            m.push(i);
        }

        let mut ret = 0;
        let mut stack = vec![0];
        visited[0] = true;
        let dest = arr.len() - 1;

        while !stack.is_empty() {
            let mut cur_size = stack.len();
            while cur_size > 0 {
                let i = stack.remove(0);
                let v = arr[i];

                if i == dest {
                    return ret;
                }

                if (i + 1 < arr.len()) && !visited[i + 1] {
                    stack.push(i + 1);
                    visited[i + 1] = true;
                }
                if (i as i32 > 0) && !visited[i - 1] {
                    stack.push(i - 1);
                    visited[i - 1] = true;
                }

                if let Some(pos) = map.get(&v) {
                    for p in pos {
                        if !visited[*p] {
                            stack.push(*p);
                            visited[*p] = true;
                        }
                    }
                    map.remove(&v);
                }
                cur_size -= 1;
            }
            ret += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1345_solution() {
        assert_eq!(
            3,
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
        );
        assert_eq!(0, Solution::min_jumps(vec![7]));
        assert_eq!(1, Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]));
        assert_eq!(2, Solution::min_jumps(vec![6, 1, 9]));
        assert_eq!(
            3,
            Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13])
        );
        assert_eq!(3, Solution::min_jumps(vec![0, 4, 3, 9]));
    }
}
