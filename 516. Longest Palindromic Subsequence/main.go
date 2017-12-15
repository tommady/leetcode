package main

import "log"

// Given a string s, find the longest palindromic subsequence's length in s.
// You may assume that the maximum length of s is 1000.

// Example 1:
// Input:

// "bbbab"
// Output:
// 4
// One possible longest palindromic subsequence is "bbbb".
// Example 2:
// Input:

// "cbbd"
// Output:
// 2
// One possible longest palindromic subsequence is "bb".

func max(m, n int) int {
	if m > n {
		return m
	}
	return n
}

func looper(s string, i, j int, memo [][]int) int {
	if memo[i][j] != 0 {
		return memo[i][j]
	}

	if i > j {
		return 0
	}

	if i == j {
		return 1
	}

	if s[i] == s[j] {
		memo[i][j] = looper(s, i+1, j-1, memo) + 2
	} else {
		memo[i][j] = max(looper(s, i+1, j, memo), looper(s, i, j-1, memo))
	}

	return memo[i][j]
}

func longestPalindromeSubseq(s string) int {
	memo := make([][]int, len(s))
	for i := range memo {
		memo[i] = make([]int, len(s))
	}

	return looper(s, 0, len(s)-1, memo)
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      int
	}{
		{
			description: "testing bbbab",
			input:       "bbbab",
			expect:      4,
		},
		{
			description: "testing cbbd",
			input:       "cbbd",
			expect:      2,
		},
		{
			description: "testing aabaa",
			input:       "aabaa",
			expect:      5,
		},
	}

	for _, testCase := range testCases {
		actual := longestPalindromeSubseq(testCase.input)
		if testCase.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", testCase.description, testCase.expect, actual)
		}
	}
}
