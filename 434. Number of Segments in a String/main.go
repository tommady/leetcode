package main

import "fmt"

func countSegments(s string) int {
	s += " "
	counter := 0
	isCounting := false

	for _, w := range s {
		if w == ' ' && isCounting {
			counter++
			isCounting = false
		} else if w != ' ' {
			isCounting = true
		}
	}
	return counter
}

func main() {
	expect := 5
	actual := countSegments("Hello, my name is John")
	if expect != actual {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
	expect = 0
	actual = countSegments("")
	if expect != actual {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
	expect = 0
	actual = countSegments("                ")
	if expect != actual {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
}
