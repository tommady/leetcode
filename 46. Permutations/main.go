package main

import (
	"log"
	"reflect"
)

// Given a collection of distinct numbers, return all possible permutations.

// For example,
// [1,2,3] have the following permutations:
// [
//   [1,2,3],
//   [1,3,2],
//   [2,1,3],
//   [2,3,1],
//   [3,1,2],
//   [3,2,1]
// ]

func dfs(nums []int, index int, ret *[][]int) {
	if index >= len(nums) {
		ns := make([]int, len(nums))
		copy(ns, nums)
		*ret = append(*ret, ns)
		return
	}

	for i := index; i < len(nums); i++ {
		nums[index], nums[i] = nums[i], nums[index]
		dfs(nums, index+1, ret)
		nums[index], nums[i] = nums[i], nums[index]
	}
}

func permute(nums []int) [][]int {
	ret := make([][]int, 0)
	dfs(nums, 0, &ret)
	return ret
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      [][]int
	}{
		{
			description: "testing 1",
			input:       []int{1, 2, 3},
			expect: [][]int{
				[]int{1, 2, 3},
				[]int{1, 3, 2},
				[]int{2, 1, 3},
				[]int{2, 3, 1},
				[]int{3, 1, 2},
				[]int{3, 2, 1},
			},
		},
	}

	for _, tt := range testCases {
		actual := permute(tt.input)
		if len(actual) != len(tt.expect) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}

		for _, ex := range tt.expect {
			found := false
			for _, ac := range actual {
				if reflect.DeepEqual(ex, ac) {
					found = true
					break
				}
			}
			if !found {
				log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
			}
		}
	}
}
