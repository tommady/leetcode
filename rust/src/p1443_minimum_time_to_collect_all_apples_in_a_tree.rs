// Given an undirected tree consisting of n vertices numbered from 0 to n-1, which has some apples in their vertices. You spend 1 second to walk over one edge of the tree. Return the minimum time in seconds you have to spend to collect all apples in the tree, starting at vertex 0 and coming back to this vertex. The edges of the undirected tree are given in the array edges, where edges[i] = [ai, bi] means that exists an edge connecting the vertices ai and bi. Additionally, there is a boolean array hasApple, where hasApple[i] = true means that vertex i has an apple; otherwise, it does not have any apple. Example 1: Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,true,true,false] Output: 8 Explanation: The figure above represents the given tree where red vertices have an apple. One optimal path to collect all apples is shown by the green arrows. Example 2: Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,false,true,false] Output: 6 Explanation: The figure above represents the given tree where red vertices have an apple. One optimal path to collect all apples is shown by the green arrows. Example 3: Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,false,false,false,false,false] Output: 0 Constraints: 1 = n = 10^5 edges.length == n - 1 edges[i].length == 2 0 = ai bi = n - 1 fromi toi hasApple.length == n
//
//
// 7
// [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]
// [false,false,true,false,true,true,false]
// 7
// [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]
// [false,false,true,false,false,true,false]
// 7
// [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]
// [false,false,false,false,false,false,false]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_time(n: i32, mut edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        edges.sort();
        let mut ret = 0;
        let mut map = vec![-1; n as usize];

        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;

            if map[to] != -1 {
                map[from] = to as i32;
            } else {
                map[to] = from as i32;
            }
        }

        for mut i in 0..has_apple.len() {
            if !has_apple[i] {
                continue;
            }

            while i != 0 && map[i] >= 0 {
                let tmp = map[i] as usize;
                map[i] = -1;
                i = tmp;
                ret += 2;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1443_solution() {
        assert_eq!(
            8,
            Solution::min_time(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                vec![false, false, true, false, true, true, false],
            )
        );
        assert_eq!(
            6,
            Solution::min_time(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                vec![false, false, true, false, false, true, false],
            )
        );
        assert_eq!(
            0,
            Solution::min_time(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 3],
                    vec![2, 6]
                ],
                vec![false, false, false, false, false, false, false],
            )
        );
        assert_eq!(
            4,
            Solution::min_time(
                4,
                vec![vec![0, 2], vec![0, 3], vec![1, 2],],
                vec![false, true, false, false],
            )
        );
        assert_eq!(
            6,
            Solution::min_time(
                4,
                vec![vec![0, 1], vec![1, 2], vec![0, 3]],
                vec![true, true, true, true]
            )
        )
    }
}
