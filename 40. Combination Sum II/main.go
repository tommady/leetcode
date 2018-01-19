package main

import (
	"log"
	"reflect"
	"sort"
)

// Given a collection of candidate numbers (C) and a target number (T),
// find all unique combinations in C where the candidate numbers sums to T.

// Each number in C may only be used once in the combination.

// Note:
// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.
// For example, given candidate set [10, 1, 2, 7, 6, 1, 5] and target 8,
// A solution set is:
// [
//   [1, 7],
//   [1, 2, 5],
//   [2, 6],
//   [1, 1, 6]
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
		if i > index && candidates[i] == candidates[i-1] {
			continue
		}
		ca := make([]int, 0)
		ca = append(ca, candidate...)
		ca = append(ca, candidates[i])
		dfs(candidates, ca, target-candidates[i], i+1, ret)
	}
}

func combinationSum2(candidates []int, target int) [][]int {
	ret := make([][]int, 0)
	sort.Ints(candidates)
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
			candidates:  []int{10, 1, 2, 7, 6, 1, 5},
			target:      8,
			expect: [][]int{
				[]int{1, 7},
				[]int{1, 2, 5},
				[]int{2, 6},
				[]int{1, 1, 6},
			},
		},
	}

	for _, tt := range testCases {
		actual := combinationSum2(tt.candidates, tt.target)

		if len(tt.expect) != len(actual) {
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
