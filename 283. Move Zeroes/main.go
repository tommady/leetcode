package main

import (
	"log"
	"reflect"
)

// Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// For example, given nums = [0, 1, 0, 3, 12], after calling your function, nums should be [1, 3, 12, 0, 0].

// Note:
// 1. You must do this in-place without making a copy of the array.
// 2. Minimize the total number of operations.

// public void moveZeroes(int[] nums) {
// 	if (nums == null || nums.length == 0) return;

// 	int insertPos = 0;
// 	for (int num: nums) {
// 	    if (num != 0) nums[insertPos++] = num;
// 	}

// 	while (insertPos < nums.length) {
// 	    nums[insertPos++] = 0;
// 	}
//     }

func moveZeroes(nums []int) {
	if nums == nil {
		return
	}
	length := len(nums)
	if length == 0 {
		return
	}

	pos := 0
	for _, n := range nums {
		if n != 0 {
			nums[pos] = n
			pos++
		}
	}

	for i := pos; i < length; i++ {
		nums[i] = 0
	}
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      []int
	}{
		{
			description: "testing 1",
			input:       []int{0, 1, 0, 3, 12},
			expect:      []int{1, 3, 12, 0, 0},
		},
	}

	for _, tt := range testCases {
		moveZeroes(tt.input)
		actual := tt.input

		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
