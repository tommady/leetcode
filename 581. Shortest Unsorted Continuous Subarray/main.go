package main

import (
	"log"
)

// Given an integer array, you need to find one continuous subarray that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order, too.

// You need to find the shortest such subarray and output its length.

// Example 1:
// Input: [2, 6, 4, 8, 10, 9, 15]
// Output: 5
// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
// Note:
// Then length of the input array is in range [1, 10,000].
// The input array may contain duplicates, so ascending order here means <=.

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func findUnsortedSubarray(nums []int) int {
	beg := -1
	end := -2
	length := len(nums)
	ma := nums[0]
	mi := nums[length-1]

	for i := 1; i < len(nums); i++ {
		mi = min(mi, nums[length-1-i])
		ma = max(ma, nums[i])

		if ma > nums[i] {
			end = i
		}
		if mi < nums[length-1-i] {
			beg = length - 1 - i
		}
	}

	return end - beg + 1
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{2, 6, 4, 8, 10, 9, 15},
			expect:      5,
		},
		{
			description: "testing 2",
			input:       []int{1, 2, 3, 4},
			expect:      0,
		},
		{
			description: "testing 3",
			input:       []int{2, 1},
			expect:      2,
		},
	}

	for _, testCase := range testCases {
		actual := findUnsortedSubarray(testCase.input)
		if testCase.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", testCase.description, testCase.expect, actual)
		}
	}
}
