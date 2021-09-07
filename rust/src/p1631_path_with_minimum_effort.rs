// You are a hiker preparing for an upcoming hike. You are given heights, a 2D array of size rows x columns, where heights[row][col] represents the height of cell (row, col). You are situated in the top-left cell, (0, 0), and you hope to travel to the bottom-right cell, (rows-1, columns-1) (i.e., 0-indexed). You can move up, down, left, or right, and you wish to find a route that requires the minimum effort.
//
//
// [[1,2,2],[3,8,2],[5,3,5]]
// [[1,2,3],[3,8,4],[5,3,5]]
// [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
//

#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;
pub struct Solution {}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        #[derive(PartialEq, Eq, Debug)]
        struct Step {
            diff: i32,
            i: usize,
            j: usize,
        }

        impl PartialOrd for Step {
            fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
                other.diff.partial_cmp(&self.diff)
            }
        }

        impl Ord for Step {
            fn cmp(&self, other: &Step) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        let mut stack = BinaryHeap::with_capacity(heights.len() * heights[0].len());
        let angles: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut ret = 0;

        let mut visited = Vec::with_capacity(heights.len() * heights[0].len());
        for _ in 0..heights.len() * heights[0].len() {
            visited.push(false);
        }

        stack.push(Step {
            diff: 0,
            i: 0,
            j: 0,
        });

        while stack.len() > 0 {
            let next = stack.pop().unwrap();
            let visit = next.i * heights[0].len() + next.j;

            if visited[visit] {
                continue;
            }

            visited[visit] = true;
            ret = ret.max(next.diff);

            if next.i == heights.len() - 1 && next.j == heights[0].len() - 1 {
                break;
            }

            for angle in angles.iter() {
                let angle_i = (next.i as i32 + angle.0) as usize;
                let angle_j = (next.j as i32 + angle.1) as usize;

                if angle_i < heights.len()
                    && angle_j < heights[0].len()
                    && angle_i as i32 >= 0
                    && angle_j as i32 >= 0
                {
                    stack.push(Step {
                        diff: (heights[angle_i][angle_j] - heights[next.i][next.j]).abs(),
                        i: angle_i,
                        j: angle_j,
                    });
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
    fn test_1631_solution() {
        assert_eq!(
            2,
            Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]])
        );
        assert_eq!(
            1,
            Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5],])
        );
        assert_eq!(
            0,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 1, 1, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 1, 2, 1],
            ])
        );
        assert_eq!(
            9,
            Solution::minimum_effort_path(vec![vec![1, 10, 6, 7, 9, 10, 4, 9]])
        );
    }
}