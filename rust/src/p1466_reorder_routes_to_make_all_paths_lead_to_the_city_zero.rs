// There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow. Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi. This year, there will be a big event in the capital (city 0), and many people want to travel to this city. Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed. It&#39;s guaranteed that each city can reach city 0 after reorder. Example 1: Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]] Output: 3 Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital). Example 2: Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]] Output: 2 Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital). Example 3: Input: n = 3, connections = [[1,0],[2,0]] Output: 0 Constraints: 2 = n = 5 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 104 connections.length == n - 1 connections[i].length == 2 0 = ai, bi = n - 1 ai != bi
//
//
// 6
// [[0,1],[1,3],[2,3],[4,0],[4,5]]
// 5
// [[1,0],[1,2],[3,2],[3,4]]
// 3
// [[1,0],[2,0]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let mut visited = vec![false; n as usize];
        let mut adjustment: Vec<Vec<(usize, i32)>> = Vec::with_capacity(n as usize);
        let mut stack = vec![0];

        for _ in 0..n as usize {
            adjustment.push(Vec::new());
        }

        for connection in connections.iter() {
            let from = connection[0] as usize;
            let to = connection[1] as usize;

            adjustment[from].push((to, 1));
            adjustment[to].push((from, 0));
        }

        while !stack.is_empty() {
            let next = stack.remove(0);
            visited[next] = true;

            for i in 0..adjustment[next].len() {
                if !visited[adjustment[next][i].0] {
                    if adjustment[next][i].1 == 1 {
                        ret += 1;
                    }
                    stack.push(adjustment[next][i].0);
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
    fn test_1466_solution() {
        // index:0 = [¿(1, 1), (4, 0)],
        // index:1 = [(0, 0), ¿(3, 1)],
        // index:2 = [(3, 1)],
        // index:3 = [(1, 0), (2, 0)],
        // index:4 = [(0, 1), ¿(5, 1)],
        // index:5 = [(4, 0)],
        assert_eq!(
            3,
            Solution::min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
            ),
        );
        // index:0 = [(1, 0)],
        // index:1 = [(0, 1), ¿(2, 1)],
        // index:2 = [(1, 0), (3, 0)],
        // index:3 = [(2, 1), ¿(4, 1)],
        // index:4 = [(3, 0)]
        assert_eq!(
            2,
            Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
        );
        // index:0 = [(1, 0), (2, 0)],
        // index:1 = [(0, 1)], [(0, 1)]
        assert_eq!(0, Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]));
        // index:0 = [(1, 0)],
        // index:1 = [¿(2, 1), (0, 1)],
        // index:2 = [¿(3, 1), (1, 0)],
        // index:3 = [(4, 0), (2, 0)],
        // index:4 = [(3, 1)]
        assert_eq!(
            2,
            Solution::min_reorder(5, vec![vec![4, 3], vec![2, 3], vec![1, 2], vec![1, 0]])
        );
    }
}
