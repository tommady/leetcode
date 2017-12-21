package main

import (
	"log"
)

// Implement strStr().

// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "hello", needle = "ll"
// Output: 2
// Example 2:

// Input: haystack = "aaaaa", needle = "bba"
// Output: -1

func strStr(haystack string, needle string) int {
	if needle == "" {
		return 0
	}

	nlen := len(needle)
	hlen := len(haystack)
	if nlen == hlen {
		if haystack == needle {
			return 0
		}
	}

	firstN := needle[0]

	for i := 0; i <= hlen-nlen; i++ {
		if haystack[i] == firstN {
			if haystack[i:i+nlen] == needle {
				return i
			}
		}
	}

	return -1
}

func main() {
	testCases := []struct {
		description string
		input1      string
		input2      string
		expect      int
	}{
		{
			description: "testing 1",
			input1:      "hello",
			input2:      "ll",
			expect:      2,
		},
		{
			description: "testing 2",
			input1:      "aaaaa",
			input2:      "bba",
			expect:      -1,
		},
		{
			description: "testing 3",
			input1:      "a",
			input2:      "",
			expect:      0,
		},
		{
			description: "testing 4",
			input1:      "",
			input2:      "a",
			expect:      -1,
		},
		{
			description: "testing 5",
			input1:      "a",
			input2:      "a",
			expect:      0,
		},
		{
			description: "testing 6",
			input1:      "mississippi",
			input2:      "pi",
			expect:      9,
		},
	}

	for _, tt := range testCases {
		actual := strStr(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", tt.description, tt.expect, actual)
		}
	}
}
