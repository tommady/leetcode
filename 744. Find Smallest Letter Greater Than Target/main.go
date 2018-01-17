package main

import "log"

// Given a list of sorted characters letters containing only lowercase letters, and given a target letter target, find the smallest element in the list that is larger than the given target.

// Letters also wrap around. For example, if the target is target = 'z' and letters = ['a', 'b'], the answer is 'a'.

// Examples:
// Input:
// letters = ["c", "f", "j"]
// target = "a"
// Output: "c"

// Input:
// letters = ["c", "f", "j"]
// target = "c"
// Output: "f"

// Input:
// letters = ["c", "f", "j"]
// target = "d"
// Output: "f"

// Input:
// letters = ["c", "f", "j"]
// target = "g"
// Output: "j"

// Input:
// letters = ["c", "f", "j"]
// target = "j"
// Output: "c"

// Input:
// letters = ["c", "f", "j"]
// target = "k"
// Output: "c"
// Note:
// letters has a length in range [2, 10000].
// letters consists of lowercase letters, and contains at least 2 unique letters.
// target is a lowercase letter.

func nextGreatestLetter(letters []byte, target byte) byte {
	n := len(letters)
	low, hig := 0, n

	for low < hig {
		mid := low + (hig-low)/2
		if letters[mid] > target {
			hig = mid
		} else {
			low = mid + 1
		}
	}

	return letters[low%n]
}

func main() {
	testCases := []struct {
		description string
		letters     []byte
		target      byte
		expect      byte
	}{
		{
			description: "testing 1",
			letters:     []byte{'c', 'f', 'j'},
			target:      'a',
			expect:      'c',
		},
		{
			description: "testing 2",
			letters:     []byte{'c', 'f', 'j'},
			target:      'c',
			expect:      'f',
		},
		{
			description: "testing 3",
			letters:     []byte{'c', 'f', 'j'},
			target:      'd',
			expect:      'f',
		},
		{
			description: "testing 4",
			letters:     []byte{'c', 'f', 'j'},
			target:      'g',
			expect:      'j',
		},
		{
			description: "testing 5",
			letters:     []byte{'c', 'f', 'j'},
			target:      'j',
			expect:      'c',
		},
		{
			description: "testing 6",
			letters:     []byte{'c', 'f', 'j'},
			target:      'k',
			expect:      'c',
		},
	}

	for _, tt := range testCases {
		actual := nextGreatestLetter(tt.letters, tt.target)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
