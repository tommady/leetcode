// Given a list of intervals, remove all intervals that are covered by another interval in the list.  Interval [a,b) is covered by interval [c,d) if and only if c = a and b = d.  After doing so, return the number of remaining intervals.   Example 1:   Input: intervals = [[1,4],[3,6],[2,8]] Output: 2 Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.   Example 2:   Input: intervals = [[1,4],[2,3]] Output: 1   Example 3:   Input: intervals = [[0,10],[5,12]] Output: 2   Example 4:   Input: intervals = [[3,10],[4,10],[5,11]] Output: 2   Example 5:   Input: intervals = [[1,2],[1,4],[3,4]] Output: 1    Constraints:   1 = intervals.length = 1000 intervals[i].length == 2 0 = intervals[i][0] intervals[i][1] = 10^5 All the intervals are unique.
//
//
// [[1,4],[3,6],[2,8]]
// [[1,4],[2,3]]
// [[0,10],[5,12]]
// [[3,10],[4,10],[5,11]]
// [[1,2],[1,4],[3,4]]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        fn inside(a: &Vec<i32>, b: &Vec<i32>) -> bool {
            if a[0] >= b[0] && a[1] <= b[1] {
                return true;
            }
            false
        }

        let mut counter: i32 = intervals.len() as i32;
        let mut i = 0;
        while i < intervals.len() {
            let mut j = 0;
            while j < intervals.len() {
                if j != i {
                    if inside(&intervals[i], &intervals[j]) {
                        counter -= 1;
                        break;
                    }
                }
                j += 1;
            }
            i += 1;
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1288_solution() {
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
        );
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]),
        );
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![0, 10], vec![5, 12]]),
        );
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![3, 10], vec![4, 10], vec![5, 11]]),
        );
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![3, 10], vec![4, 10], vec![5, 11]]),
        );
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]]),
        );
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![
                vec![34335, 39239],
                vec![15875, 91969],
                vec![29673, 66453],
                vec![53548, 69161],
                vec![40618, 93111],
            ]),
        );
    }
}
