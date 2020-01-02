package main

import "fmt"

func main() {
	input := []int{1}
	expect := 1
	actual := singleNumber(input)
	if expect != actual {
		fmt.Printf("input:%v\n", input)
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
	input = []int{2, 2, 1}
	expect = 1
	actual = singleNumber(input)
	if expect != actual {
		fmt.Printf("input:%v\n", input)
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
	input = []int{1, 0, 1}
	expect = 0
	actual = singleNumber(input)
	if expect != actual {
		fmt.Printf("input:%v\n", input)
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
}

func singleNumber(nums []int) int {
	for i := 1; i < len(nums); i++ {
		nums[0] = nums[0] ^ nums[i]
	}
	return nums[0]
}
