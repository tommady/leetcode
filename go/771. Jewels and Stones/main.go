package main

import "log"

// You're given strings J representing the types of stones that are jewels, and S representing the stones you have.  Each character in S is a type of stone you have.  You want to know how many of the stones you have are also jewels.

// The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are case sensitive, so "a" is considered a different type of stone from "A".

// Example 1:

// Input: J = "aA", S = "aAAbbbb"
// Output: 3
// Example 2:

// Input: J = "z", S = "ZZ"
// Output: 0
// Note:

// S and J will consist of letters and have length at most 50.
// The characters in J are distinct.

func numJewelsInStones(J string, S string) int {
	jm := make(map[rune]struct{})
	for _, j := range J {
		jm[j] = struct{}{}
	}

	ret := 0
	for _, s := range S {
		if _, exist := jm[s]; exist {
			ret++
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		J           string
		S           string
		expect      int
	}{
		{
			description: "testing 1",
			J:           "aA",
			S:           "aAAbbbb",
			expect:      3,
		},
		{
			description: "testing 2",
			J:           "z",
			S:           "ZZ",
			expect:      0,
		},
	}

	for _, tt := range testCases {
		actual := numJewelsInStones(tt.J, tt.S)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
