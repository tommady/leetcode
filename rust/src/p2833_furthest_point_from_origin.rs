// You are given a string moves of length n consisting only of characters 'L', 'R', and '_'. The string represents your movement on a number line starting from the origin 0.
// In the ith move, you can choose one of the following directions:
//  move to the left if moves[i] = 'L' or moves[i] = '_' move to the right if moves[i] = 'R' or moves[i] = '_'  Return the
// distance from the origin of the
// furthest point you can get to after n moves.
//
// Example 1:
//
// Input: moves = "L_RL__R"
// Output: 3
// Explanation: The furthest point we can reach from the origin 0 is point -3 through the following sequence of moves "LLRLLLR".  Example 2:
//
// Input: moves = "_R__LL_"
// Output: 5
// Explanation: The furthest point we can reach from the origin 0 is point -5 through the following sequence of moves "LRLLLLL".  Example 3:
//
// Input: moves = "_______"
// Output: 7
// Explanation: The furthest point we can reach from the origin 0 is point 7 through the following sequence of moves "RRRRRRR".
//
// Constraints:
//  1 = moves.length == n = 50 moves consists only of characters 'L', 'R' and '_'.

// hint 1
// In an optimal answer, all occurrences of '_’ will be replaced with the same character.

// hint 2
// Replace all characters of '_’ with the character that occurs the most.

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        // 0 = L, 1 = R, 2 = _
        let mut m = [0; 3];
        for mov in moves.chars() {
            match mov {
                'L' => m[0] += 1i32,
                'R' => m[1] += 1i32,
                _ => m[2] += 1i32,
            }
        }

        if m[2] as usize == moves.len() {
            return m[2];
        }
        if m[0] == m[1] {
            return m[0] + m[2] - m[1];
        }
        if m[0] > m[1] {
            return m[0] + m[2] - m[1];
        }
        m[1] + m[2] - m[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2833_solution() {
        assert_eq!(
            3,
            Solution::furthest_distance_from_origin("L_RL__R".to_string())
        );
        assert_eq!(
            5,
            Solution::furthest_distance_from_origin("_R__LL_".to_string())
        );
        assert_eq!(
            7,
            Solution::furthest_distance_from_origin("_______".to_string())
        );
    }
}
