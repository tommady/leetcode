// Given two sequences pushed and popped with distinct values, return true if and only if this could have been the result of a sequence of push and pop operations on an initially empty stack. Example 1: Input: pushed = [1,2,3,4,5], popped = [4,5,3,2,1] Output: true Explanation: We might do the following sequence: push(1), push(2), push(3), push(4), pop() - 4, push(5), pop() - 5, pop() - 3, pop() - 2, pop() - 1 Example 2: Input: pushed = [1,2,3,4,5], popped = [4,3,5,1,2] Output: false Explanation: 1 cannot be popped before 2. Constraints: 0 = pushed.length == popped.length = 1000 0 = pushed[i], popped[i] 1000 pushed is a permutation of popped. pushed and popped have distinct values.
//
//
// [1,2,3,4,5]
// [4,5,3,2,1]
// [1,2,3,4,5]
// [4,3,5,1,2]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::with_capacity(pushed.len());
        let mut stack_index = 0;
        stack.push(-1);
        let mut pushed_index = 0;

        for pop in popped.iter() {
            while stack[stack_index] != *pop && pushed_index < pushed.len() {
                stack.push(pushed[pushed_index]);
                pushed_index += 1;
                stack_index += 1;
            }

            if *pop != stack.pop().unwrap() {
                return false;
            }
            stack_index -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_946_solution() {
        assert!(Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1]
        ));
        assert!(!Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 3, 5, 1, 2]
        ));
        assert!(Solution::validate_stack_sequences(vec![0, 1], vec![0, 1]))
    }
}
