package main

import (
	"log"
	"reflect"
)

// Given a set of candidate numbers (C) (without duplicates) and a target number (T),
// find all unique combinations in C where the candidate numbers sums to T.

// The same repeated number may be chosen from C unlimited number of times.

// Note:
// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.
// For example, given candidate set [2, 3, 6, 7] and target 7,
// A solution set is:
// [
//   [7],
//   [2, 2, 3]
// ]

func dfs(candidates, candidate []int, target, index int, ret *[][]int) {
	if target < 0 {
		return
	}

	if target == 0 {
		*ret = append(*ret, candidate)
		return
	}

	for i := index; i < len(candidates); i++ {
		ca := make([]int, 0)
		ca = append(ca, candidate...)
		ca = append(ca, candidates[i])
		dfs(candidates, ca, target-candidates[i], i, ret)
	}
}

func combinationSum(candidates []int, target int) [][]int {
	ret := make([][]int, 0)
	dfs(candidates, []int{}, target, 0, &ret)
	return ret
}

func main() {
	testCases := []struct {
		description string
		candidates  []int
		target      int
		expect      [][]int
	}{
		{
			description: "testing 1",
			candidates:  []int{2, 3, 6, 7},
			target:      7,
			expect:      [][]int{[]int{7}, []int{2, 2, 3}},
		},
	}

	for _, tt := range testCases {
		actual := combinationSum(tt.candidates, tt.target)
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
