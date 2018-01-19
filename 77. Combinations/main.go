package main

import (
	"log"
	"reflect"
)

// Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.

// For example,
// If n = 4 and k = 2, a solution is:

// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]

func dfs(index, n, k int, comb []int, ret *[][]int) {
	if k == 0 {
		*ret = append(*ret, comb)
		return
	}

	for i := index; i <= n; i++ {
		co := make([]int, 0)
		co = append(co, comb...)
		co = append(co, i)
		dfs(i+1, n, k-1, co, ret)
	}
}

func combine(n int, k int) [][]int {
	ret := make([][]int, 0)
	dfs(1, n, k, []int{}, &ret)
	return ret
}

func main() {
	testCases := []struct {
		description string
		n           int
		k           int
		expect      [][]int
	}{
		{
			description: "testing 1",
			n:           4,
			k:           2,
			expect: [][]int{
				[]int{2, 4},
				[]int{3, 4},
				[]int{2, 3},
				[]int{1, 2},
				[]int{1, 3},
				[]int{1, 4},
			},
		},
	}

	for _, tt := range testCases {
		actual := combine(tt.n, tt.k)

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
