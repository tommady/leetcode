package main

import (
	"fmt"
	"reflect"
)

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func findDisappearedNumbers(nums []int) []int {
	for _, n := range nums {
		if nums[abs(n)-1] > 0 {
			nums[abs(n)-1] = -nums[abs(n)-1]
		}
	}
	ret := make([]int, 0, len(nums))
	for i, n := range nums {
		if n > 0 {
			ret = append(ret, i+1)
		}
	}
	return ret
}

func main() {
	expect := []int{5, 6}
	actual := findDisappearedNumbers([]int{4, 3, 2, 7, 8, 2, 3, 1})
	if !reflect.DeepEqual(expect, actual) {
		fmt.Println("expect and actual are not the same")
		fmt.Printf("expect:%v\n", expect)
		fmt.Printf("actual:%v\n", actual)
	}
}
