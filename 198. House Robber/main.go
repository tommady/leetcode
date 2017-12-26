package main

import (
	"log"
)

// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

// Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func rob(nums []int) int {
	a, b := 0, 0
	for i, n := range nums {
		if i&1 == 0 {
			b = max(a, b+n)
		} else {
			a = max(a+n, b)
		}
	}

	return max(a, b)
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{1, 2, 3, 4, 5, 6},
			expect:      12,
		},
		{
			description: "testing 2",
			input:       []int{2, 1, 1, 2},
			expect:      4,
		},
	}

	for _, tt := range testCases {
		actual := rob(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%d != actual:%d", tt.description, tt.expect, actual)
		}
	}
}
