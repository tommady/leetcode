package main

import "log"

// Given two strings s and t, write a function to determine if t is an anagram of s.

// For example,
// s = "anagram", t = "nagaram", return true.
// s = "rat", t = "car", return false.

// Note:
// You may assume the string contains only lowercase alphabets.

// Follow up:
// What if the inputs contain unicode characters? How would you adapt your solution to such case?

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	sm := make(map[rune]int)
	tm := make(map[rune]int)

	for i := 0; i < len(s); i++ {
		sw := rune(s[i])
		tw := rune(t[i])
		sm[sw]++
		tm[tw]++
	}
	for key, count := range sm {
		if c := tm[key]; c != count {
			return false
		}
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
			input1:      "anagram",
			input2:      "nagaram",
			expect:      true,
		},
		{
			description: "testing 2",
			input1:      "rat",
			input2:      "car",
			expect:      false,
		},
		{
			description: "testing 3",
			input1:      "ab",
			input2:      "a",
			expect:      false,
		},
	}

	for _, tt := range testCases {
		actual := isAnagram(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
