package main

import (
	"fmt"
)

func hammingDistance(x int, y int) (ret int) {
	v := x ^ y

	for v != 0 {
		ret++
		v &= v - 1
	}
	return
}

func main() {
	expect := 2
	actual := hammingDistance(1, 4)
	if expect != actual {
		fmt.Printf("error expect:%v, actual:%v\n", expect, actual)
	}
}
