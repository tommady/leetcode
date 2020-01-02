package main

import (
	"log"
	"reflect"
)

// You are given an n x n 2D matrix representing an image.

// Rotate the image by 90 degrees (clockwise).

// Note:
// You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.

// Example 1:

// Given input matrix =
// [
//   [1,2,3],
//   [4,5,6],
//   [7,8,9]
// ],

// rotate the input matrix in-place such that it becomes:
// [
//   [7,4,1],
//   [8,5,2],
//   [9,6,3]
// ]
// Example 2:

// Given input matrix =
// [
//   [ 5, 1, 9,11],
//   [ 2, 4, 8,10],
//   [13, 3, 6, 7],
//   [15,14,12,16]
// ],

// rotate the input matrix in-place such that it becomes:
// [
//   [15,13, 2, 5],
//   [14, 3, 4, 1],
//   [12, 6, 8, 9],
//   [16, 7,10,11]
// ]

func rotate(matrix [][]int) {
	last := len(matrix) - 1
	for i := 0; i < len(matrix)/2; i++ {
		matrix[i], matrix[last-i] = matrix[last-i], matrix[i]
	}

	for i := 0; i < len(matrix); i++ {
		for j := i + 1; j < len(matrix[i]); j++ {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}
}

func main() {
	testCases := []struct {
		description string
		input       [][]int
		expect      [][]int
	}{
		{
			description: "testing 1",
			input: [][]int{
				[]int{1, 2, 3},
				[]int{4, 5, 6},
				[]int{7, 8, 9},
			},
			expect: [][]int{
				[]int{7, 4, 1},
				[]int{8, 5, 2},
				[]int{9, 6, 3},
			},
		},
		{
			description: "testing 2",
			input: [][]int{
				[]int{5, 1, 9, 11},
				[]int{2, 4, 8, 10},
				[]int{13, 3, 6, 7},
				[]int{15, 14, 12, 16},
			},
			expect: [][]int{
				[]int{15, 13, 2, 5},
				[]int{14, 3, 4, 1},
				[]int{12, 6, 8, 9},
				[]int{16, 7, 10, 11},
			},
		},
	}

	for _, tt := range testCases {
		rotate(tt.input)
		actual := tt.input
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect :%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
