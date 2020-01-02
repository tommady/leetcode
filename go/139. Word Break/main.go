package main

import (
	"log"
)

// Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words. You may assume the dictionary does not contain duplicate words.

// For example, given
// s = "leetcode",
// dict = ["leet", "code"].

// Return true because "leetcode" can be segmented as "leet code".

// UPDATE (2017/1/4):
// The wordDict parameter had been changed to a list of strings (instead of a set of strings). Please reload the code definition to get the latest changes.

func wordBreak(s string, wordDict []string) bool {
	m := make([]bool, len(s)+1)
	m[0] = true

	for i := 1; i <= len(s); i++ {
		for _, wd := range wordDict {
			if len(wd) <= i && m[i-len(wd)] {
				if s[i-len(wd):i] == wd {
					m[i] = true
					break
				}
			}
		}
	}

	return m[len(s)]
}

func main() {
	testCases := []struct {
		description string
		input1      string
		input2      []string
		expect      bool
	}{
		{
			description: "testing 1",
			input1:      "leetcode",
			input2:      []string{"leet", "code"},
			expect:      true,
		},
		{
			description: "testing 2",
			input1:      "bb",
			input2:      []string{"a", "b", "bbb", "bbbb"},
			expect:      true,
		},
		{
			description: "testing 3",
			input1:      "abcd",
			input2:      []string{"a", "abc", "b", "cd"},
			expect:      true,
		},
		{
			description: "testing 4",
			input1:      "aaaaaaaa",
			input2:      []string{"aaaa", "aa", "a"},
			expect:      true,
		},
		{
			description: "testing 5",
			input1:      "aaaaaaa",
			input2:      []string{"aaaa", "aa"},
			expect:      false,
		},
	}

	for _, testCase := range testCases {
		actual := wordBreak(testCase.input1, testCase.input2)
		if testCase.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", testCase.description, testCase.expect, actual)
		}
	}
}
