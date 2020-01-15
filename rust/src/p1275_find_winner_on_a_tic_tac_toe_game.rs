// Tic-tac-toe is played by two players A and B on a 3 x 3 grid.
//
// Here are the rules of Tic-Tac-Toe:
//
//
// 	Players take turns placing characters into empty squares ( ).
// 	The first player A always places X characters, while the second player B always places O characters.
// 	 X and O characters are always placed into empty squares, never on filled ones.
// 	The game ends when there are 3 of the same (non-empty) character filling any row, column, or diagonal.
// 	The game also ends if all squares are non-empty.
// 	No more moves can be played if the game is over.
//
//
// Given an array moves where each element is another array of size 2 corresponding to the row and column of the grid where they mark their respective character in the order in which A and B play.
//
// Return the winner of the game if it exists (A or B), in case the game ends in a draw return Draw , if there are still movements to play return Pending .
//
// You can assume that moves is valid (It follows the rules of Tic-Tac-Toe), the grid is initially empty and A will play first.
//
//
// Example 1:
//
//
// Input: moves = [[0,0],[2,0],[1,1],[2,1],[2,2]]
// Output: A
// Explanation: A wins, he always plays first.
// X X X X X
// - - X - X - X
// O O OO OOX
//
//
// Example 2:
//
//
// Input: moves = [[0,0],[1,1],[0,1],[0,2],[1,0],[2,0]]
// Output: B
// Explanation: B wins.
// X X XX XXO XXO XXO
// - O - O - O - XO - XO
// O
//
//
// Example 3:
//
//
// Input: moves = [[0,0],[1,1],[2,0],[1,0],[1,2],[2,1],[0,1],[0,2],[2,2]]
// Output: Draw
// Explanation: The game ends in a draw since there are no moves to make.
// XXO
// OOX
// XOX
//
//
// Example 4:
//
//
// Input: moves = [[0,0],[1,1]]
// Output: Pending
// Explanation: The game has not finished yet.
// X
// O
//
//
//
//
// Constraints:
//
//
// 	1 = moves.length = 9
// 	moves[i].length == 2
// 	0 = moves[i][j] = 2
// 	There are no repeated elements on moves.
// 	moves follow the rules of tic tac toe.
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board: [&str; 9] = [""; 9];

        for i in 0..moves.len() {
            let x = moves[i][0] as usize;
            let y = moves[i][1] as usize;
            let index = x * 3 + y;
            if i % 2 == 0 {
                board[index] = "A";
            } else {
                board[index] = "B";
            }

            for mark in &["A".to_owned(), "B".to_owned()] {
                if (board[0] == mark && board[1] == mark && board[2] == mark)
                    || (board[3] == mark && board[4] == mark && board[5] == mark)
                    || (board[6] == mark && board[7] == mark && board[8] == mark)
                    || (board[0] == mark && board[4] == mark && board[8] == mark)
                    || (board[2] == mark && board[4] == mark && board[6] == mark)
                    || (board[0] == mark && board[3] == mark && board[6] == mark)
                    || (board[1] == mark && board[4] == mark && board[7] == mark)
                    || (board[2] == mark && board[5] == mark && board[8] == mark)
                {
                    return mark.to_owned();
                }
            }
        }

        for b in board.iter() {
            if *b == "" {
                return "Pending".to_owned();
            }
        }

        "Draw".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1275_solution() {
        // "X  "    "X  "    "X  "    "X  "    "X  "
        // "   " -> "   " -> " X " -> " X " -> " X "
        // "   "    "O  "    "O  "    "OO "    "OOX"
        assert_eq!(
            "A",
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ])
        );
        // "X  "    "X  "    "XX "    "XXO"    "XXO"    "XXO"
        // "   " -> " O " -> " O " -> " O " -> "XO " -> "XO "
        // "   "    "   "    "   "    "   "    "   "    "O  "
        assert_eq!(
            "B",
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ])
        );
        // "XXO"
        // "OOX"
        // "XOX"
        assert_eq!(
            "Draw",
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ])
        );
        // "X  "
        // " O "
        // "   "
        assert_eq!("Pending", Solution::tictactoe(vec![vec![0, 0], vec![1, 1]]));
        // "XOX"
        // "XOX"
        // "XO "
        assert_eq!(
            "B",
            Solution::tictactoe(vec![
                vec![2, 0],
                vec![1, 1],
                vec![0, 2],
                vec![2, 1],
                vec![1, 2],
                vec![1, 0],
                vec![0, 0],
                vec![0, 1]
            ])
        );
    }
}
