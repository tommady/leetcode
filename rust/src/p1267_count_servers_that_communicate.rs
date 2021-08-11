// You are given a map of a server center, represented as a m Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n integer matrix grid, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column. Return the number of servers that communicate with any other server. Example 1: Input: grid = [[1,0],[0,1]] Output: 0 Explanation: No servers can communicate with others. Example 2: Input: grid = [[1,0],[1,1]] Output: 3 Explanation: All three servers can communicate with at least one other server. Example 3: Input: grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]] Output: 4 Explanation: The two servers in the first row can communicate with each other. The two servers in the third column can communicate with each other. The server at right bottom corner can&#39;t communicate with any other server. Constraints: m == grid.length n == grid[i].length 1 = m = 250 1 = n = 250 grid[i][j] == 0 or 1
//
//
// [[1,0],[0,1]]
// [[1,0],[1,1]]
// [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let n = grid.len();
        let m = grid[0].len();
        let mut row = vec![0; n];
        let mut col = vec![0; m];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 && (row[i] > 1 || col[j] > 1) {
                    ret += 1;
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
    fn test_1267_solution() {
        assert_eq!(0, Solution::count_servers(vec![vec![1, 0], vec![0, 1]]));
        assert_eq!(3, Solution::count_servers(vec![vec![1, 0], vec![1, 1]]));
        assert_eq!(
            4,
            Solution::count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ])
        );
    }
}
