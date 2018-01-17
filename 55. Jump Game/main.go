package main

import "log"

// Given an array of non-negative integers, you are initially positioned at the first index of the array.

// Each element in the array represents your maximum jump length at that position.

// Determine if you are able to reach the last index.

// For example:
// A = [2,3,1,1,4], return true.

// A = [3,2,1,0,4], return false.

func canJump(nums []int) bool {
	pos := 0
	for i := 0; i < len(nums); i++ {
		if i > pos {
			return false
		}

		reach := i + nums[i]
		if pos < reach {
			pos = reach
		}
	}
	return true
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      bool
	}{
		{
			description: "testing 1",
			input:       []int{2, 3, 1, 1, 4},
			expect:      true,
		},
		{
			description: "testing 2",
			input:       []int{3, 2, 1, 0, 4},
			expect:      false,
		},
		{
			description: "testing 3",
			input:       []int{0},
			expect:      true,
		},
		{
			description: "testing 4",
			input:       []int{2, 0},
			expect:      true,
		},
		{
			description: "testing 5",
			input:       []int{2, 5, 0, 0},
			expect:      true,
		},
		{
			description: "testing 6",
			input:       []int{0, 2, 3},
			expect:      false,
		},
		{
			description: "testing 7",
			input:       []int{3, 0, 8, 2, 0, 0, 1},
			expect:      true,
		},
		{
			description: "testing 8",
			input:       []int{1, 0, 1, 0},
			expect:      false,
		},
	}

	for _, tt := range testCases {
		actual := canJump(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
