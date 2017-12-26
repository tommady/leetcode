package main

import "log"

// Given an array of integers and an integer k,
// find out whether there are two distinct indices i and j in the array such that nums[i] = nums[j]
// and the absolute difference between i and j is at most k.

func containsNearbyDuplicate(nums []int, k int) bool {
	m := make(map[int]int)
	for i, n := range nums {
		if dis, exist := m[n]; exist && i-dis <= k {
			return true
		}
		m[n] = i
	}

	return false
}

func main() {
	testCases := []struct {
		description string
		input1      []int
		input2      int
		expect      bool
	}{
		{
			description: "testing 1",
			input1:      []int{1, 2, 3, 1},
			input2:      3,
			expect:      true,
		},
	}

	for _, tt := range testCases {
		actual := containsNearbyDuplicate(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
