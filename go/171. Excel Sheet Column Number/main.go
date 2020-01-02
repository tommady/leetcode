package main

import "log"

// Related to question Excel Sheet Column Title

// Given a column title as appear in an Excel sheet, return its corresponding column number.

// For example:

//     A -> 1
//     B -> 2
//     C -> 3
//     ...
//     Z -> 26
//     AA -> 27
//     AB -> 28

func titleToNumber(s string) int {
	ret := 0
	for _, w := range s {
		ret = ret*26 + (int(w-'A') + 1)
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      int
	}{
		{
			description: "testing 1",
			input:       "A",
			expect:      1,
		},
		{
			description: "testing 2",
			input:       "C",
			expect:      3,
		},
		{
			description: "testing 3",
			input:       "Z",
			expect:      26,
		},
		{
			description: "testing 4",
			input:       "AA",
			expect:      27,
		},
		{
			description: "testing 5",
			input:       "AB",
			expect:      28,
		},
		{
			description: "testing 6",
			input:       "BA",
			expect:      53,
		},
	}

	for _, tt := range testCases {
		actual := titleToNumber(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%d] != actual[%d]", tt.description, tt.expect, actual)
		}
	}
}
