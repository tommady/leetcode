package main

import (
	"log"
	"math"
)

// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You may assume no duplicates in the array.

// Example 1:

// Input: [1,3,5,6], 5
// Output: 2
// Example 2:

// Input: [1,3,5,6], 2
// Output: 1
// Example 3:

// Input: [1,3,5,6], 7
// Output: 4
// Example 4:

// Input: [1,3,5,6], 0
// Output: 0

func searchInsert(nums []int, target int) int {
	posiblePos := math.MinInt16

	for i, n := range nums {
		if target <= n {
			posiblePos = i
			break
		}
	}

	if posiblePos == math.MinInt16 {
		posiblePos = len(nums)
	}

	return posiblePos
}

func main() {
	testCases := []struct {
		description string
		input1      []int
		input2      int
		expect      int
	}{
		{
			description: "testing 1",
			input1:      []int{1, 3, 5, 6},
			input2:      5,
			expect:      2,
		},
		{
			description: "testing 2",
			input1:      []int{1, 3, 5, 6},
			input2:      2,
			expect:      1,
		},
		{
			description: "testing 3",
			input1:      []int{1, 3, 5, 6},
			input2:      7,
			expect:      4,
		},
		{
			description: "testing 4",
			input1:      []int{1, 3, 5, 6},
			input2:      0,
			expect:      0,
		},
	}

	for _, tt := range testCases {
		actual := searchInsert(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", tt.description, tt.expect, actual)
		}
	}
}
