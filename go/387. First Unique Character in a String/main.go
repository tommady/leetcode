package main

import "log"

// Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.

// Examples:

// s = "leetcode"
// return 0.

// s = "loveleetcode",
// return 2.
// Note: You may assume the string contain only lowercase letters.

func firstUniqChar(s string) int {
	ct := make([]int32, 26)

	for _, c := range s {
		ct[c-'a']++
	}

	for i, c := range s {
		if ct[c-'a'] == 1 {
			return i
		}
	}

	return -1
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      int
	}{
		{
			description: "testing leetcode",
			input:       "leetcode",
			expect:      0,
		},
		{
			description: "testing loveleetcode",
			input:       "loveleetcode",
			expect:      2,
		},
		{
			description: "testing ttttttttt",
			input:       "ttttttttt",
			expect:      -1,
		},
	}

	for _, testCase := range testCases {
		actual := firstUniqChar(testCase.input)
		if testCase.expect != actual {
			log.Fatalf("%s: expect %d != actual %d", testCase.description, testCase.expect, actual)
		}
	}
}
