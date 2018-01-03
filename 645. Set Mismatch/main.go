package main

import (
	"log"
	"reflect"
)

// The set S originally contains numbers from 1 to n. But unfortunately, due to the data error, one of the numbers in the set got duplicated to another number in the set, which results in repetition of one number and loss of another number.

// Given an array nums representing the data status of this set after the error. Your task is to firstly find the number occurs twice and then find the number that is missing. Return them in the form of an array.

// Example 1:
// Input: nums = [1,2,2,4]
// Output: [2,3]
// Note:
// The given array size will in the range [2, 10000].
// The given array's numbers won't have any order.

func findErrorNums(nums []int) []int {
	m := make([]int, len(nums)+1)
	dup := 0
	sum := 0
	for _, n := range nums {
		m[n]++
		sum += n
		if m[n] > 1 {
			dup = n
		}
	}

	osum := (1 + len(nums)) * len(nums) / 2
	mis := osum - sum + dup

	return []int{dup, mis}
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      []int
	}{
		{
			description: "testing 1",
			input:       []int{1, 2, 2, 4},
			expect:      []int{2, 3},
		},
		{
			description: "testing 2",
			input:       []int{1, 1},
			expect:      []int{1, 2},
		},
		{
			description: "testing 3",
			input:       []int{2, 2},
			expect:      []int{2, 1},
		},
	}

	for _, tt := range testCases {
		actual := findErrorNums(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
