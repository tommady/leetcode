package main

import "log"

// Calculate the sum of two integers a and b, but you are not allowed to use the operator + and -.

// Example:
// Given a = 1 and b = 2, return 3.

func getSum(a int, b int) int {
	for b != 0 {
		carry := a & b
		a ^= b
		b = carry << 1
	}
	return a
}

func main() {
	testCases := []struct {
		description string
		input1      int
		input2      int
		expect      int
	}{
		{
			description: "testing 1",
			input1:      1,
			input2:      2,
			expect:      3,
		},
		{
			description: "testing 2",
			input1:      100,
			input2:      200,
			expect:      300,
		},
	}

	for _, tt := range testCases {
		actual := getSum(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
