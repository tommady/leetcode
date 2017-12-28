package main

import (
	"log"
	"reflect"
)

// Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others appear once.

// Find all the elements of [1, n] inclusive that do not appear in this array.

// Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

// Example:

// Input:
// [4,3,2,7,8,2,3,1]

// Output:
// [5,6]

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func findDisappearedNumbers(nums []int) []int {
	for _, n := range nums {
		if nums[abs(n)-1] > 0 {
			nums[abs(n)-1] = -nums[abs(n)-1]
		}
	}

	ret := make([]int, 0, len(nums))
	for i, n := range nums {
		if n > 0 {
			ret = append(ret, i+1)
		}
	}
	return ret
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      []int
	}{
		{
			description: "testing 1",
			input:       []int{4, 3, 2, 7, 8, 2, 3, 1},
			expect:      []int{5, 6},
		},
	}

	for _, tt := range testCases {
		actual := findDisappearedNumbers(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
