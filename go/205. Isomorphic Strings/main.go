package main

import (
	"log"
)

// Given two strings s and t, determine if they are isomorphic.

// Two strings are isomorphic if the characters in s can be replaced to get t.

// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character but a character may map to itself.

// For example,
// Given "egg", "add", return true.

// Given "foo", "bar", return false.

// Given "paper", "title", return true.

// Note:
// You may assume both s and t have the same length.

func isIsomorphic(s string, t string) bool {
	smap := make(map[rune]int)
	tmap := make(map[rune]int)

	for i := 0; i < len(s); i++ {
		sw := rune(s[i])
		tw := rune(t[i])

		if smap[sw] != tmap[tw] {
			return false
		}

		// plus one to avoid index 0 makes the counting not correct
		smap[sw] += i + 1
		tmap[tw] += i + 1
	}

	return true
}

func main() {
	testCases := []struct {
		description string
		input1      string
		input2      string
		expect      bool
	}{
		{
			description: "testing 1",
			input1:      "egg",
			input2:      "add",
			expect:      true,
		},
		{
			description: "testing 2",
			input1:      "foo",
			input2:      "bar",
			expect:      false,
		},
		{
			description: "testing 3",
			input1:      "paper",
			input2:      "title",
			expect:      true,
		},
		{
			description: "testing 4",
			input1:      "abba",
			input2:      "abab",
			expect:      false,
		},
		{
			description: "testing 5",
			input1:      "ab",
			input2:      "aa",
			expect:      false,
		},
	}

	for _, tt := range testCases {
		actual := isIsomorphic(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
