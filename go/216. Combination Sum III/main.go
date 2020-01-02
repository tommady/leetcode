package main

import (
	"fmt"
	"reflect"
)

// Find all possible combinations of k numbers that add up to a number n,
// given that only numbers from 1 to 9 can be used and each combination should be a unique set of numbers.

// Example 1:

// Input: k = 3, n = 7

// Output:

// [[1,2,4]]

// Example 2:

// Input: k = 3, n = 9

// Output:

// [[1,2,6], [1,3,5], [2,3,4]]

func combinationSum3(k, n int) [][]int {
	ans := make([][]int, 0)
	combination(&ans, []int{}, k, 1, n)
	return ans
}

func combination(ans *[][]int, comb []int, k, start, n int) {
	if len(comb) == k && n == 0 {
		newComb := make([]int, len(comb))
		copy(newComb, comb)
		*ans = append(*ans, newComb)
		return
	}
	for i := start; i <= 9; i++ {
		comb = append(comb, i)
		combination(ans, comb, k, i+1, n-i)
		comb = comb[:len(comb)-1]
	}
}

func main() {
	expect := [][]int{
		[]int{1, 2, 4},
	}
	actual := combinationSum3(3, 7)
	if !reflect.DeepEqual(expect, actual) {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
	expect = [][]int{
		[]int{1, 2, 6},
		[]int{1, 3, 5},
		[]int{2, 3, 4},
	}
	actual = combinationSum3(3, 9)
	if !reflect.DeepEqual(expect, actual) {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
}
