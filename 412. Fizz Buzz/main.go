package main

// Write a program that outputs the string representation of numbers from 1 to n.

// But for multiples of three it should output “Fizz” instead of the number and for the multiples of five output “Buzz”. For numbers which are multiples of both three and five output “FizzBuzz”.

// Example:

// n = 15,

// Return:
// [
//     "1",
//     "2",
//     "Fizz",
//     "4",
//     "Buzz",
//     "Fizz",
//     "7",
//     "8",
//     "Fizz",
//     "Buzz",
//     "11",
//     "Fizz",
//     "13",
//     "14",
//     "FizzBuzz"
// ]

import (
	"fmt"
	"reflect"
	"strconv"
)

func fizzBuzz(n int) []string {
	ret := make([]string, n)
	fizz, buzz := 0, 0

	for i := 1; i <= n; i++ {
		var s string
		fizz++
		buzz++

		if fizz == 3 {
			s += "Fizz"
			fizz = 0
		}
		if buzz == 5 {
			s += "Buzz"
			buzz = 0
		}
		if s == "" {
			s = strconv.Itoa(i)
		}
		ret[i-1] = s
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
