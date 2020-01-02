package main

import (
	"log"
	"math"
)

// Given a non-empty array of integers, return the third maximum number in this array.
// If it does not exist, return the maximum number. The time complexity must be in O(n).

// Example 1:
// Input: [3, 2, 1]

// Output: 1

// Explanation: The third maximum is 1.
// Example 2:
// Input: [1, 2]

// Output: 2

// Explanation: The third maximum does not exist, so the maximum (2) is returned instead.
// Example 3:
// Input: [2, 2, 3, 1]

// Output: 1

// Explanation: Note that the third maximum here means the third maximum distinct number.
// Both numbers with value 2 are both considered as second maximum.

func thirdMax(nums []int) int {
	one := math.MinInt64
	two := math.MinInt64
	three := math.MinInt64

	for _, n := range nums {
		if one == n || two == n || three == n {
			continue
		}

		if n > one {
			three = two
			two = one
			one = n
		} else if n > two {
			three = two
			two = n

		} else if n > three {
			three = n
		}
	}

	if three == math.MinInt64 {
		return one
	}

	return three
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      int
	}{
		{
			description: "testing 1",
			input:       []int{3, 2, 1},
			expect:      1,
		},
		{
			description: "testing 2",
			input:       []int{1, 2},
			expect:      2,
		},
		{
			description: "testing 3",
			input:       []int{2, 2, 3, 1},
			expect:      1,
		},
		{
			description: "testing 4",
			input:       []int{1, 2, -2147483648},
			expect:      -2147483648,
		},
	}

	for _, tt := range testCases {
		actual := thirdMax(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
