package main

import (
	"log"
	"reflect"
)

// Given an array of integers that is already sorted in ascending order, find two numbers such that they add up to a specific target number.

// The function twoSum should return indices of the two numbers such that they add up to the target, where index1 must be less than index2. Please note that your returned answers (both index1 and index2) are not zero-based.

// You may assume that each input would have exactly one solution and you may not use the same element twice.

// Input: numbers={2, 7, 11, 15}, target=9
// Output: index1=1, index2=2

func twoSum(numbers []int, target int) []int {
	beg, end := 0, len(numbers)-1

	for beg != end {
		sum := numbers[beg] + numbers[end]
		if sum < target {
			beg++
		} else if sum > target {
			end--
		} else {
			return []int{beg + 1, end + 1}
		}
	}

	return nil
}

func main() {
	testCases := []struct {
		description string
		input1      []int
		input2      int
		expect      []int
	}{
		{
			description: "testing 1",
			input1:      []int{2, 7, 11, 15},
			input2:      9,
			expect:      []int{1, 2},
		},
	}

	for _, tt := range testCases {
		actual := twoSum(tt.input1, tt.input2)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect[%v] != actual[%v]", tt.description, tt.expect, actual)
		}
	}
}
