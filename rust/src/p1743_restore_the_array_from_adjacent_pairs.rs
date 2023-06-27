// There is an integer array nums that consists of n unique elements, but you have forgotten it. However, you do remember every pair of adjacent elements in nums. You are given a 2D integer array adjacentPairs of size n - 1 where each adjacentPairs[i] = [ui, vi] indicates that the elements ui and vi are adjacent in nums. It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in adjacentPairs, either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear in any order. Return the original array nums. If there are multiple solutions, return any of them. Example 1: Input: adjacentPairs = [[2,1],[3,4],[3,2]] Output: [1,2,3,4] Explanation: This array has all its adjacent pairs in adjacentPairs. Notice that adjacentPairs[i] may not be in left-to-right order. Example 2: Input: adjacentPairs = [[4,-2],[1,4],[-3,1]] Output: [-2,4,1,-3] Explanation: There can be negative numbers. Another solution is [-3,1,4,-2], which would also be accepted. Example 3: Input: adjacentPairs = [[100000,-100000]] Output: [100000,-100000] Constraints: nums.length == n adjacentPairs.length == n - 1 adjacentPairs[i].length == 2 2 = n = 105 -105 = nums[i], ui, vi = 105 There exists some nums that has adjacentPairs as its pairs.
//
//
// [[2,1],[3,4],[3,2]]
// [[4,-2],[1,4],[-3,1]]
// [[100000,-100000]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut ret: Vec<i32> = Vec::with_capacity(adjacent_pairs.len() + 1);
        ret.resize(adjacent_pairs.len() + 1, 0);
        let mut map: HashMap<i32, Vec<i32>> = HashMap::with_capacity(adjacent_pairs.len() - 1);

        for pair in &adjacent_pairs {
            let a = pair[0];
            let b = pair[1];
            map.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
            map.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        }

        for (k, vec) in &map {
            if vec.len() == 1 {
                ret[0] = *k;
                ret[1] = vec[0];
                break;
            }
        }

        for i in 2..adjacent_pairs.len() + 1 {
            if let Some(v) = map.get(&ret[i - 1]) {
                ret[i] = v[0];
                if v[0] == ret[i - 2] {
                    ret[i] = v[1]
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1743_solution() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]])
        );

        let s2 = Solution::restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]);
        let a2 = s2 == vec![-2, 4, 1, -3] || s2 == vec![-3, 1, 4, -2];
        assert!(a2);
    }
}
