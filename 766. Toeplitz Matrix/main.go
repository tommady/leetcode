package main

import "log"

// A matrix is Toeplitz if every diagonal from top-left to bottom-right has the same element.

// Now given an M x N matrix, return True if and only if the matrix is Toeplitz.

// Example 1:

// Input: matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
// Output: True
// Explanation:
// 1234
// 5123
// 9512

// In the above grid, the diagonals are "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]", and in each diagonal all elements are the same, so the answer is True.
// Example 2:

// Input: matrix = [[1,2],[2,2]]
// Output: False
// Explanation:
// The diagonal "[1, 2]" has different elements.
// Note:

// matrix will be a 2D array of integers.
// matrix will have a number of rows and columns in range [1, 20].
// matrix[i][j] will be integers in range [0, 99].

func isToeplitzMatrix(matrix [][]int) bool {
	for i := 0; i < len(matrix)-1; i++ {
		for j := 0; j < len(matrix[i])-1; j++ {
			if matrix[i][j] != matrix[i+1][j+1] {
				return false
			}
		}
	}

	return true
}

func main() {
	testCases := []struct {
		description string
		input       [][]int
		expect      bool
	}{
		{
			description: "testing 1",
			input: [][]int{
				[]int{1, 2, 3, 4},
				[]int{5, 1, 2, 3},
				[]int{9, 5, 1, 2},
			},
			expect: true,
		},
		// {
		// 	description: "testing 2",
		// 	input: [][]int{
		// 		[]int{1, 2},
		// 		[]int{2, 2},
		// 	},
		// 	expect: false,
		// },
	}

	for _, tt := range testCases {
		actual := isToeplitzMatrix(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
