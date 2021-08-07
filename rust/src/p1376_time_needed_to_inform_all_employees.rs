// A company has n employees with a unique ID for each employee from 0 to n - 1. The head of the company is the one with headID. Each employee has one direct manager given in the manager array where manager[i] is the direct manager of the i-th employee, manager[headID] = -1. Also, it is guaranteed that the subordination relationships have a tree structure. The head of the company wants to inform all the company employees of an urgent piece of news. He will inform his direct subordinates, and they will inform their subordinates, and so on until all employees know about the urgent news. The i-th employee needs informTime[i] minutes to inform all of his direct subordinates (i.e., After informTime[i] minutes, all his direct subordinates can start spreading the news). Return the number of minutes needed to inform all the employees about the urgent news. Example 1: Input: n = 1, headID = 0, manager = [-1], informTime = [0] Output: 0 Explanation: The head of the company is the only employee in the company. Example 2: Input: n = 6, headID = 2, manager = [2,2,-1,2,2,2], informTime = [0,0,1,0,0,0] Output: 1 Explanation: The head of the company with id = 2 is the direct manager of all the employees in the company and needs 1 minute to inform them all. The tree structure of the employees in the company is shown. Example 3: Input: n = 7, headID = 6, manager = [1,2,3,4,5,6,-1], informTime = [0,6,5,4,3,2,1] Output: 21 Explanation: The head has id = 6. He will inform employee with id = 5 in 1 minute. The employee with id = 5 will inform the employee with id = 4 in 2 minutes. The employee with id = 4 will inform the employee with id = 3 in 3 minutes. The employee with id = 3 will inform the employee with id = 2 in 4 minutes. The employee with id = 2 will inform the employee with id = 1 in 5 minutes. The employee with id = 1 will inform the employee with id = 0 in 6 minutes. Needed time = 1 + 2 + 3 + 4 + 5 + 6 = 21. Example 4: Input: n = 15, headID = 0, manager = [-1,0,0,1,1,2,2,3,3,4,4,5,5,6,6], informTime = [1,1,1,1,1,1,1,0,0,0,0,0,0,0,0] Output: 3 Explanation: The first minute the head will inform employees 1 and 2. The second minute they will inform employees 3, 4, 5 and 6. The third minute they will inform the rest of employees. Example 5: Input: n = 4, headID = 2, manager = [3,3,-1,2], informTime = [0,0,162,914] Output: 1076 Constraints: 1 = n = 105 0 = headID n manager.length == n 0 = manager[i] n manager[headID] == -1 informTime.length == n 0 = informTime[i] = 1000 informTime[i] == 0 if employee i has no subordinates. It is guaranteed that all the employees can be informed.
//
//
// 1
// 0
// [-1]
// [0]
// 6
// 2
// [2,2,-1,2,2,2]
// [0,0,1,0,0,0]
// 7
// 6
// [1,2,3,4,5,6,-1]
// [0,6,5,4,3,2,1]
// 15
// 0
// [-1,0,0,1,1,2,2,3,3,4,4,5,5,6,6]
// [1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]
// 4
// 2
// [3,3,-1,2]
// [0,0,162,914]
//

#![allow(dead_code)]

pub struct Solution {}

// Runtime: 32 ms, faster than 100.00% of Rust online submissions for Time Needed to Inform All Employees.
// Memory Usage: 3.5 MB, less than 66.67% of Rust online submissions for Time Needed to Inform All Employees.
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut visited = vec![0; n as usize];

        for i in 0..n as usize {
            let mut i_inform = 0;
            let mut labor = manager[i];

            while labor != -1 {
                let labor_i = labor as usize;

                i_inform += inform_time[labor_i];
                if visited[labor_i] >= i_inform {
                    break;
                }
                visited[labor_i] = i_inform;
                labor = manager[labor_i];
            }
        }

        visited[head_id as usize]
    }
}

use std::collections::HashMap;
pub struct Solution1 {}

// Runtime: 100 ms, faster than 33.33% of Rust online submissions for Time Needed to Inform All Employees.
// Memory Usage: 12.1 MB, less than 33.33% of Rust online submissions for Time Needed to Inform All Employees.
impl Solution1 {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..n as usize {
            map.entry(manager[i]).or_insert(Vec::new()).push(i as i32);
        }

        let mut stack = vec![];
        for labor in map.get(&-1).unwrap() {
            stack.push((*labor, inform_time[head_id as usize]));
        }

        while stack.len() > 0 {
            let (you, i_inform) = stack.remove(0);

            if i_inform > ret {
                ret = i_inform;
            }

            if let Some(labors) = map.get(&you) {
                for i in 0..labors.len() {
                    let labor = labors[i] as usize;
                    stack.push((labors[i], inform_time[labor] + i_inform));
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
    fn test_1376_solution() {
        assert_eq!(0, Solution::num_of_minutes(1, 0, vec![-1], vec![0]));
        assert_eq!(
            1,
            Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0])
        );
        assert_eq!(
            21,
            Solution::num_of_minutes(7, 6, vec![1, 2, 3, 4, 5, 6, -1], vec![0, 6, 5, 4, 3, 2, 1])
        );
        assert_eq!(
            3,
            Solution::num_of_minutes(
                15,
                0,
                vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
                vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
            )
        );
        assert_eq!(
            1076,
            Solution::num_of_minutes(4, 2, vec![3, 3, -1, 2], vec![0, 0, 162, 914])
        );
    }
}
