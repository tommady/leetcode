package main

import (
	"log"
	"strings"
)

// Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.

// If the last word does not exist, return 0.

// Note: A word is defined as a character sequence consists of non-space characters only.

// Example:

// Input: "Hello World"
// Output: 5

func lengthOfLastWord(s string) int {
	wds := strings.Split(s, " ")
	for i := len(wds) - 1; i >= 0; i-- {
		if wds[i] != "" {
			return len(wds[i])
		}
	}
	return 0
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      int
	}{
		{
			description: "testing 1",
			input:       "Hello World",
			expect:      5,
		},
		{
			description: "testing 2",
			input:       "a ",
			expect:      1,
		},
		{
			description: "testing 3",
			input:       " a",
			expect:      1,
		},
		{
			description: "testing 4",
			input:       " a ",
			expect:      1,
		},
	}

	for _, tt := range testCases {
		actual := lengthOfLastWord(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", tt.description, tt.expect, actual)
		}
	}
}
