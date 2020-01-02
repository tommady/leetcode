package main

import "log"

// Given a word, you need to judge whether the usage of capitals in it is right or not.

// We define the usage of capitals in a word to be right when one of the following cases holds:

// All letters in this word are capitals, like "USA".
// All letters in this word are not capitals, like "leetcode".
// Only the first letter in this word is capital if it has more than one letter, like "Google".
// Otherwise, we define that this word doesn't use capitals in a right way.
// Example 1:
// Input: "USA"
// Output: True
// Example 2:
// Input: "FlaG"
// Output: False
// Note: The input will be a non-empty word consisting of uppercase and lowercase latin letters.

func detectCapitalUse(word string) bool {
	counter := 0
	for _, w := range word {
		if 'Z'-w >= 0 {
			counter++
		}
	}

	if counter == len(word) || counter == 0 || (counter == 1 && 'Z'-rune(word[0]) >= 0) {
		return true
	}

	return false
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      bool
	}{
		// {
		// 	description: "testing 1",
		// 	input:       "USA",
		// 	expect:      true,
		// },
		// {
		// 	description: "testing 2",
		// 	input:       "FlaG",
		// 	expect:      false,
		// },
		{
			description: "testing 3",
			input:       "ffffffffffffffffffffF",
			expect:      false,
		},
	}

	for _, tt := range testCases {
		actual := detectCapitalUse(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
