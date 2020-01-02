package main

import (
	"fmt"
)

func reverseString(s string) string {
	if len(s) <= 1 {
		return s
	}

	i, j := 0, len(s)-1
	ret := make([]byte, len(s))

	for i <= j {
		ret[i] = s[j]
		ret[j] = s[i]
		i++
		j--
	}

	return string(ret)
}

func main() {
	expect := "olleh"
	actual := reverseString("hello")
	if expect != actual {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
}
