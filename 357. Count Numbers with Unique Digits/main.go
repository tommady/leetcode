package main

import "log"

// Given a non-negative integer n, count all numbers with unique digits, x, where 0 ≤ x < 10n.

// Example:
// Given n = 2, return 91.
// (The answer should be the total numbers in the range of 0 ≤ x < 100, excluding [11,22,33,44,55,66,77,88,99])

func countNumbersWithUniqueDigits(n int) int {
	if n == 0 {
		return 1
	}

	ret := 10
	udigits := 9
	availableNum := 9

	for n > 1 && availableNum > 0 {
		udigits = udigits * availableNum
		ret += udigits
		availableNum--
		n--
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       int
		expect      int
	}{
		{
			description: "testing1",
			input:       2,
			expect:      91,
		},
	}

	for _, testCase := range testCases {
		actual := countNumbersWithUniqueDigits(testCase.input)
		if testCase.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", testCase.description, testCase.expect, actual)
		}
	}
}
