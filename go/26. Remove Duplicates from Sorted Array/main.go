package main

import "log"

// Given a sorted array, remove the duplicates in-place such that each element appear only once and return the new length.

// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

// Example:

// Given nums = [1,1,2],

// Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
// It doesn't matter what you leave beyond the new length.

func removeDuplicates(nums []int) int {
	length := len(nums)
	if length < 2 {
		return length
	}

	c := 1
	for i := 1; i < length; i++ {
		if nums[i] != nums[i-1] {
			nums[c] = nums[i]
			c++
		}
	}

	return c
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{1, 1, 2},
			expect:      2,
		},
		{
			description: "testing 2",
			input:       []int{0, 0, 0, 0, 0},
			expect:      0,
		},
	}

	for _, tt := range testCases {
		actual := removeDuplicates(tt.input)
		if tt.expect != actual {
			log.Fatalf(
				"%s: input:%v, expect[%v] != actual[%v]",
				tt.description,
				tt.input,
				tt.expect,
				actual,
			)
		}
	}
}
