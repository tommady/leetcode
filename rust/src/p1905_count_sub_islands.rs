// You are given two m x n binary matrices grid1 and grid2 containing only 0&#39;s (representing water) and 1&#39;s (representing land). An island is a group of 1&#39;s connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells. An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2. Return the number of islands in grid2 that are considered sub-islands. Example 1: Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]] Output: 3 Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2. The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands. Example 2: Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]] Output: 2 Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2. The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands. Constraints: m == grid1.length == grid2.length n == grid1[i].length == grid2[i].length 1 = m, n = 500 grid1[i][j] and grid2[i][j] are either 0 or 1.
//
//
// [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]]
// [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
// [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]]
// [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let mut i = 0;
        let mut j = 0;
        let angles: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut stack = vec![];

        while i < grid1.len() && j < grid2[i].len() {
            if grid2[i][j] != 1 {
                j += 1;
                if j == grid2[i].len() {
                    i += 1;
                    j = 0;
                }
                continue;
            }

            stack.clear();
            stack.push((i as i32, j as i32));

            let mut is_inside = true;
            while stack.len() > 0 {
                let (angle_i, angle_j) = stack.remove(0);
                grid2[angle_i as usize][angle_j as usize] = -1;

                if grid1[angle_i as usize][angle_j as usize] != 1 {
                    is_inside = false;
                }

                for angle in angles.iter() {
                    let angle0 = (angle.0 + angle_i) as usize;
                    let angle1 = (angle.1 + angle_j) as usize;

                    if angle0 < grid1.len()
                        && angle0 as i32 >= 0
                        && angle1 < grid2[angle0].len()
                        && angle1 as i32 >= 0
                    {
                        if grid2[angle0][angle1] == 1 {
                            stack.push((angle0 as i32, angle1 as i32));
                            grid2[angle0][angle1] = -1;
                        }
                    }
                }
            }

            if is_inside {
                ret += 1;
            }
        }

        ret
    }
}

fn pretty_print(g: &Vec<Vec<i32>>) {
    println!("THE GRID");
    for i in g.iter() {
        for j in i.iter() {
            print!("{}\t", j);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1905_solution() {
        assert_eq!(
            3,
            Solution::count_sub_islands(
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 1, 1],
                ],
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 0, 1, 1, 1],
                    vec![0, 1, 0, 0, 0],
                    vec![1, 0, 1, 1, 0],
                    vec![0, 1, 0, 1, 0],
                ]
            )
        );
        assert_eq!(
            2,
            Solution::count_sub_islands(
                vec![
                    vec![1, 0, 1, 0, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 0, 1, 0, 1],
                ],
                vec![
                    vec![0, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 1, 0, 1, 0],
                    vec![0, 1, 0, 1, 0],
                    vec![1, 0, 0, 0, 1],
                ]
            )
        );
        assert_eq!(
            2,
            Solution::count_sub_islands(
                vec![
                    vec![0, 1, 1, 1, 1, 1, 1, 0, 1, 1],
                    vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 1],
                    vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1],
                    vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1],
                    vec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1],
                    vec![1, 1, 0, 0, 1, 1, 1, 0, 0, 1],
                    vec![1, 1, 0, 1, 1, 0, 0, 1, 1, 0],
                    vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 0, 0, 1, 1, 0, 1, 1, 1, 1]
                ],
                vec![
                    vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 0, 0, 1, 1, 1, 0, 0, 1, 0],
                    vec![1, 1, 1, 0, 1, 1, 0, 1, 1, 1],
                    vec![1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
                    vec![0, 1, 1, 1, 0, 1, 0, 1, 1, 0],
                    vec![1, 1, 1, 0, 0, 0, 1, 0, 1, 0],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 0, 0, 0, 1, 0, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 0, 1, 1, 0]
                ],
            )
        )
    }
}
