package main

import (
	"log"
	"reflect"
)

// Given an array of integers sorted in ascending order, find the starting and ending position of a given target value.

// Your algorithm's runtime complexity must be in the order of O(log n).

// If the target is not found in the array, return [-1, -1].

// For example,
// Given [5, 7, 7, 8, 8, 10] and target value 8,
// return [3, 4].

func bsearch(nums []int, target int) int {
	low, hig := 0, len(nums)

	for low < hig {
		mid := low + (hig-low)>>1

		if nums[mid] < target {
			low = mid + 1
		} else {
			hig = mid
		}
	}

	return low
}

func searchRange(nums []int, target int) []int {
	beg := bsearch(nums, target)
	if beg == len(nums) || nums[beg] != target {
		return []int{-1, -1}
	}

	return []int{beg, bsearch(nums, target+1) - 1}
}

func main() {
	testCases := []struct {
		description string
		nums        []int
		target      int
		expect      []int
	}{
		{
			description: "testing 1",
			nums:        []int{5, 7, 7, 8, 8, 10},
			target:      8,
			expect:      []int{3, 4},
		},
		{
			description: "testing 2",
			nums:        []int{1},
			target:      1,
			expect:      []int{0, 0},
		},
	}

	for _, tt := range testCases {
		actual := searchRange(tt.nums, tt.target)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
