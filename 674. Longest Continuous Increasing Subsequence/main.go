package main

import "log"

// Given an unsorted array of integers, find the length of longest continuous increasing subsequence (subarray).

// Example 1:
// Input: [1,3,5,4,7]
// Output: 3
// Explanation: The longest continuous increasing subsequence is [1,3,5], its length is 3.
// Even though [1,3,5,7] is also an increasing subsequence, it's not a continuous one where 5 and 7 are separated by 4.
// Example 2:
// Input: [2,2,2,2,2]
// Output: 1
// Explanation: The longest continuous increasing subsequence is [2], its length is 1.
// Note: Length of the array will not exceed 10,000.

func findLengthOfLCIS(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	ret, counter := 1, 1
	for i := 1; i < len(nums); i++ {
		if nums[i-1] < nums[i] {
			counter++
			if counter > ret {
				ret = counter
			}
		} else {
			counter = 1
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{1, 3, 5, 4, 7},
			expect:      3,
		},
		{
			description: "testing 2",
			input:       []int{2, 2, 2, 2, 2},
			expect:      1,
		},
	}

	for _, tt := range testCases {
		actual := findLengthOfLCIS(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
