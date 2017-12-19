package main

import (
	"log"
)

// Given an array with n integers, your task is to check if it could become non-decreasing by modifying at most 1 element.

// We define an array is non-decreasing if array[i] <= array[i + 1] holds for every i (1 <= i < n).

// Example 1:
// Input: [4,2,3]
// Output: True
// Explanation: You could modify the first 4 to 1 to get a non-decreasing array.

// Example 2:
// Input: [4,2,1]
// Output: False
// Explanation: You can't get a non-decreasing array by modify at most one element.

// Note: The n belongs to [1, 10,000].

func checkPossibility(nums []int) bool {
	if len(nums) <= 2 {
		return true
	}
	changed := 0
	for i := 1; i < len(nums) && changed <= 1; i++ {
		if nums[i] < nums[i-1] {
			changed++
			if i >= 2 && nums[i] < nums[i-2] {
				nums[i] = nums[i-1]
			}
		}
	}
	if changed > 1 {
		return false
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
			input:       []int{4, 2, 3},
			expect:      true,
		},
		{
			description: "testing 2",
			input:       []int{4, 2, 1},
			expect:      false,
		},
		{
			description: "testing 3",
			input:       []int{-1, 4, 2, 3},
			expect:      true,
		},
		{
			description: "testing 4",
			input:       []int{3, 4, 2, 3},
			expect:      false,
		},
		{
			description: "testing 5",
			input:       []int{1},
			expect:      true,
		},
		{
			description: "testing 6",
			input:       []int{3, 1},
			expect:      true,
		},
	}

	for _, tt := range testCases {
		actual := checkPossibility(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect(%v) != actual(%v)", tt.description, tt.expect, actual)
		}
	}
}
