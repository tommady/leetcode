package main

import (
	"log"
	"reflect"
)

// A self-dividing number is a number that is divisible by every digit it contains.

// For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.

// Also, a self-dividing number is not allowed to contain the digit zero.

// Given a lower and upper number bound, output a list of every possible self dividing number, including the bounds if possible.

// Example 1:
// Input:
// left = 1, right = 22
// Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
// Note:

// The boundaries of each input argument are 1 <= left <= right <= 10000.

func selfDividingNumbers(left int, right int) []int {
	ret := make([]int, 0)
	for i := left; i <= right; i++ {
		num := i
		isDiv := true

		for num > 0 {
			d := num % 10
			if d == 0 || i%d != 0 {
				isDiv = false
				break
			}
			num /= 10
		}

		if isDiv {
			ret = append(ret, i)
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		left        int
		right       int
		expect      []int
	}{
		{
			description: "testing 1",
			left:        1,
			right:       22,
			expect:      []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22},
		},
	}

	for _, tt := range testCases {
		actual := selfDividingNumbers(tt.left, tt.right)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
