package main

import "log"

// Given a 2D board and a word, find if the word exists in the grid.

// The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.

// For example,
// Given board =

// [
//   ['A','B','C','E'],
//   ['S','F','C','S'],
//   ['A','D','E','E']
// ]
// word = "ABCCED", -> returns true,
// word = "SEE", -> returns true,
// word = "ABCB", -> returns false.

func exist(board [][]byte, word string) bool {
	ws := make([]byte, len(word))
	for i, w := range word {
		ws[i] = byte(w)
	}

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if found(board, i, j, 0, ws) {
				return true
			}
		}
	}

	return false
}

func found(board [][]byte, i, j, windex int, ws []byte) bool {
	if windex >= len(ws) {
		return true
	}
	if i < 0 || j < 0 || i >= len(board) || j >= len(board[i]) {
		return false
	}
	if board[i][j] != ws[windex] {
		return false
	}

	board[i][j] ^= 255

	exist := found(board, i+1, j, windex+1, ws) ||
		found(board, i-1, j, windex+1, ws) ||
		found(board, i, j+1, windex+1, ws) ||
		found(board, i, j-1, windex+1, ws)

	board[i][j] ^= 255

	return exist
}

func main() {
	testCases := []struct {
		description string
		board       [][]byte
		word        string
		expect      bool
	}{
		{
			description: "testing 1",
			board: [][]byte{
				[]byte{'A', 'B', 'C', 'E'},
				[]byte{'S', 'F', 'C', 'S'},
				[]byte{'A', 'D', 'E', 'E'},
			},
			word:   "ABCCED",
			expect: true,
		},
		{
			description: "testing 2",
			board: [][]byte{
				[]byte{'A', 'B', 'C', 'E'},
				[]byte{'S', 'F', 'C', 'S'},
				[]byte{'A', 'D', 'E', 'E'},
			},
			word:   "SEE",
			expect: true,
		},
		{
			description: "testing 3",
			board: [][]byte{
				[]byte{'A', 'B', 'C', 'E'},
				[]byte{'S', 'F', 'C', 'S'},
				[]byte{'A', 'D', 'E', 'E'},
			},
			word:   "ABCB",
			expect: false,
		},
		{
			description: "testing 4",
			board: [][]byte{
				[]byte{'a'},
			},
			word:   "ab",
			expect: false,
		},
		{
			description: "testing 5",
			board: [][]byte{
				[]byte{'a', 'b'},
				[]byte{'c', 'd'},
			},
			word:   "abcd",
			expect: false,
		},
		{
			description: "testing 6",
			board: [][]byte{
				[]byte{'C', 'A', 'A'},
				[]byte{'A', 'A', 'A'},
				[]byte{'B', 'C', 'D'},
			},
			word:   "AAB",
			expect: true,
		},
	}

	for _, tt := range testCases {
		actual := exist(tt.board, tt.word)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
