package main

import "log"

// Given an array containing n distinct numbers taken from 0, 1, 2, ..., n, find the one that is missing from the array.

// Example 1

// Input: [3,0,1]
// Output: 2

// Example 2

// Input: [9,6,4,2,3,5,7,0,1]
// Output: 8

// Note:
// Your algorithm should run in linear runtime complexity. Could you implement it using only constant extra space complexity?

func missingNumber(nums []int) int {
	length := len(nums)

	// 0,1,2,3,5 = (0+5) * (5+1) / 2
	sum := (0 + length) * (length + 1) / 2
	for _, n := range nums {
		sum -= n
	}

	return sum
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{3, 0, 1},
			expect:      2,
		},
		{
			description: "testing 2",
			input:       []int{9, 6, 4, 2, 3, 5, 7, 0, 1},
			expect:      8,
		},
	}

	for _, tt := range testCases {
		actual := missingNumber(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
