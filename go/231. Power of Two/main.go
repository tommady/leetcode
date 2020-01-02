package main

import "log"

func isPowerOfTwo(n int) bool {
	if n <= 0 {
		return false
	}

	return (n & (n - 1)) == 0
}

func main() {
	testCases := []struct {
		description string
		input       int
		expect      bool
	}{
		{
			description: "testing input 4",
			input:       4,
			expect:      true,
		},
		{
			description: "testing input 6",
			input:       6,
			expect:      false,
		},
		{
			description: "testing input 11",
			input:       11,
			expect:      false,
		},
	}

	for _, testCase := range testCases {
		actual := isPowerOfTwo(testCase.input)
		if testCase.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", testCase.description, testCase.expect, actual)
		}
	}
}
