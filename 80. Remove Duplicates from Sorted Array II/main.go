package main

import "log"

// Follow up for "Remove Duplicates":
// What if duplicates are allowed at most twice?

// For example,
// Given sorted array nums = [1,1,1,2,2,3],

// Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2 and 3. It doesn't matter what you leave beyond the new length.

func removeDuplicates(nums []int) int {
	n, counter := len(nums), 0
	for i := 2; i < n; i++ {
		if nums[i] == nums[i-2-counter] {
			counter++
		} else {
			nums[i-counter] = nums[i]
		}
	}

	return n - counter
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{1, 1, 1, 2, 2, 3},
			expect:      5,
		},
	}

	for _, tt := range testCases {
		actual := removeDuplicates(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
