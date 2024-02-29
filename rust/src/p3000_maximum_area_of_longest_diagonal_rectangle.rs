// You are given a 2D
// 0-indexed integer array dimensions.
// For all indices i, 0 = i dimensions.length, dimensions[i][0] represents the length and dimensions[i][1] represents the width of the rectangle i.
// Return the
// area of the rectangle having the
// longest diagonal. If there are multiple rectangles with the longest diagonal, return the area of the rectangle having the
// maximum area.
//
// Example 1:
//
// Input: dimensions = [[9,3],[8,6]]
// Output: 48
// Explanation: For index = 0, length = 9 and width = 3. Diagonal length = sqrt(9 Cargo.lock Cargo.toml Justfile rustfmt.toml src target template.rs 9 + 3 Cargo.lock Cargo.toml Justfile rustfmt.toml src target template.rs 3) = sqrt(90)  9.487. For index = 1, length = 8 and width = 6. Diagonal length = sqrt(8 Cargo.lock Cargo.toml Justfile rustfmt.toml src target template.rs 8 + 6 Cargo.lock Cargo.toml Justfile rustfmt.toml src target template.rs 6) = sqrt(100) = 10. So, the rectangle at index 1 has a greater diagonal length therefore we return area = 8 Cargo.lock Cargo.toml Justfile rustfmt.toml src target template.rs 6 = 48.  Example 2:
//
// Input: dimensions = [[3,4],[4,3]]
// Output: 12
// Explanation: Length of diagonal is the same for both which is 5, so maximum area = 12.
//
// Constraints:
//  1 = dimensions.length = 100 dimensions[i].length == 2 1 = dimensions[i][0], dimensions[i][1] = 100

// hint 1
// Diagonal of rectangle is sqrt(length2 + width2).

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_diagonal = 0;
        let mut max_area = 0;

        for dimension in dimensions {
            let diagonal = dimension[0] * dimension[0] + dimension[1] * dimension[1];
            let area = dimension[0] * dimension[1];

            if diagonal > max_diagonal || (diagonal == max_diagonal && area > max_area) {
                max_diagonal = diagonal;
                max_area = area;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_3000_solution() {
        assert_eq!(
            48,
            Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]])
        );
        assert_eq!(
            12,
            Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]])
        );
        assert_eq!(
            20,
            Solution::area_of_max_diagonal(vec![
                vec![6, 5],
                vec![8, 6],
                vec![2, 10],
                vec![8, 1],
                vec![9, 2],
                vec![3, 5],
                vec![3, 5]
            ])
        );
        assert_eq!(
            2028,
            Solution::area_of_max_diagonal(vec![
                vec![4, 7],
                vec![8, 9],
                vec![5, 3],
                vec![6, 10],
                vec![2, 9],
                vec![3, 10],
                vec![2, 2],
                vec![5, 8],
                vec![5, 10],
                vec![5, 6],
                vec![8, 9],
                vec![10, 7],
                vec![8, 9],
                vec![3, 7],
                vec![2, 6],
                vec![5, 1],
                vec![7, 4],
                vec![1, 10],
                vec![1, 7],
                vec![6, 9],
                vec![3, 3],
                vec![4, 6],
                vec![8, 2],
                vec![10, 6],
                vec![7, 9],
                vec![9, 2],
                vec![1, 2],
                vec![3, 8],
                vec![10, 2],
                vec![4, 1],
                vec![9, 7],
                vec![10, 3],
                vec![6, 9],
                vec![9, 8],
                vec![7, 7],
                vec![5, 7],
                vec![5, 4],
                vec![6, 5],
                vec![1, 8],
                vec![2, 3],
                vec![7, 10],
                vec![3, 9],
                vec![5, 7],
                vec![2, 4],
                vec![5, 6],
                vec![9, 5],
                vec![8, 8],
                vec![8, 10],
                vec![6, 8],
                vec![5, 1],
                vec![10, 8],
                vec![7, 4],
                vec![2, 1],
                vec![2, 7],
                vec![10, 3],
                vec![2, 5],
                vec![7, 6],
                vec![10, 5],
                vec![10, 9],
                vec![5, 7],
                vec![10, 6],
                vec![4, 3],
                vec![10, 4],
                vec![1, 5],
                vec![8, 9],
                vec![3, 1],
                vec![2, 5],
                vec![9, 10],
                vec![6, 6],
                vec![5, 10],
                vec![10, 2],
                vec![6, 10],
                vec![1, 1],
                vec![8, 6],
                vec![1, 7],
                vec![6, 3],
                vec![9, 3],
                vec![1, 4],
                vec![1, 1],
                vec![10, 4],
                vec![7, 9],
                vec![4, 5],
                vec![2, 8],
                vec![7, 9],
                vec![7, 3],
                vec![4, 9],
                vec![2, 8],
                vec![4, 6],
                vec![9, 1],
                vec![8, 4],
                vec![2, 4],
                vec![7, 8],
                vec![3, 5],
                vec![7, 6],
                vec![8, 6],
                vec![4, 7],
                vec![25, 60],
                vec![39, 52],
                vec![16, 63],
                vec![33, 56]
            ])
        );
    }
}
