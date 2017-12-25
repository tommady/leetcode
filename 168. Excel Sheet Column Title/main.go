package main

import "log"

// Given a positive integer, return its corresponding column title as appear in an Excel sheet.

// For example:

//     1 -> A
//     2 -> B
//     3 -> C
//     ...
//     26 -> Z
//     27 -> AA
//     28 -> AB

func convertToTitle(n int) string {
	ret := ""
	for n > 0 {
		n--
		ret = string('A'+(n%26)) + ret
		n /= 26
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       int
		expect      string
	}{
		{
			description: "testing 1",
			input:       1,
			expect:      "A",
		},
		{
			description: "testing 2",
			input:       26,
			expect:      "Z",
		},
		{
			description: "testing 3",
			input:       27,
			expect:      "AA",
		},
		{
			description: "testing 4",
			input:       28,
			expect:      "AB",
		},
	}

	for _, tt := range testCases {
		actual := convertToTitle(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%s] != actual[%s]", tt.description, tt.expect, actual)
		}
	}
}
