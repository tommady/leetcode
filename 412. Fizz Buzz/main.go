package main

import (
	"fmt"
	"reflect"
	"strconv"
)

func fizzBuzz(n int) []string {
	ret := make([]string, 0, n)

	for i := 1; i <= n; i++ {
		if i%3 == 0 && i%5 == 0 {
			ret = append(ret, "FizzBuzz")
		} else if i%3 == 0 {
			ret = append(ret, "Fizz")
		} else if i%5 == 0 {
			ret = append(ret, "Buzz")
		} else {
			ret = append(ret, strconv.Itoa(i))
		}
	}
	return ret
}

func main() {
	expect := []string{
		"1",
		"2",
		"Fizz",
		"4",
		"Buzz",
		"Fizz",
		"7",
		"8",
		"Fizz",
		"Buzz",
		"11",
		"Fizz",
		"13",
		"14",
		"FizzBuzz",
	}
	actual := fizzBuzz(15)
	if !reflect.DeepEqual(expect, actual) {
		fmt.Println("expect and actual are not the same")
		fmt.Printf("expect:%v\n", expect)
		fmt.Printf("actual:%v\n", actual)
	}
}
