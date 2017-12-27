package main

import (
	"log"
)

// Given an integer array nums, find the sum of the elements between indices i and j (i ≤ j), inclusive.

// Example:
// Given nums = [-2, 0, 3, -5, 2, -1]

// sumRange(0, 2) -> 1
// sumRange(2, 5) -> -1
// sumRange(0, 5) -> -3
// Note:
// 1. You may assume that the array does not change.
// 2. There are many calls to sumRange function.

// NumArray implement this question's operations.
type NumArray struct {
	sums []int
}

// Constructor returns a NumArray instance.
func Constructor(nums []int) NumArray {
	sums := make([]int, len(nums)+1)
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		sums[i+1] = sum
	}

	return NumArray{
		sums: sums,
	}
}

// SumRange returns the sum of the ranges of nums.
func (n *NumArray) SumRange(i int, j int) int {
	return n.sums[j+1] - n.sums[i]
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.SumRange(i,j);
 */

func main() {
	nums := []int{-2, 0, 3, -5, 2, -1}
	obj := Constructor(nums)

	expect := 1
	actual := obj.SumRange(0, 2)
	if expect != actual {
		log.Fatalf("expect:%v != actual:%v", expect, actual)
	}

	expect = -1
	actual = obj.SumRange(2, 5)
	if expect != actual {
		log.Fatalf("expect:%v != actual:%v", expect, actual)
	}

	expect = -3
	actual = obj.SumRange(0, 5)
	if expect != actual {
		log.Fatalf("expect:%v != actual:%v", expect, actual)
	}
}
