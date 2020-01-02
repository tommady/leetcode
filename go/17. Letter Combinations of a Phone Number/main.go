package main

import (
	"log"
)

// Given a digit string, return all possible letter combinations that the number could represent.

// A mapping of digit to letters (just like on the telephone buttons) is given below.

// Input:Digit string "23"
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
// Note:
// Although the above answer is in lexicographical order, your answer could be in any order you want.

var (
	mm = map[byte]string{
		'0': "0",
		'1': "1",
		'2': "abc",
		'3': "def",
		'4': "ghi",
		'5': "jkl",
		'6': "mno",
		'7': "pqrs",
		'8': "tuv",
		'9': "wxyz",
	}
)

func letterCombinations(digits string) []string {
	ret := make([]string, 0)
	if len(digits) == 0 {
		return ret
	}

	for _, c := range mm[digits[0]] {
		ret = append(ret, string(c))
	}

	for i := 1; i < len(digits); i++ {
		tmp := make([]string, 0)
		for _, c := range mm[digits[i]] {
			for _, r := range ret {
				tmp = append(tmp, string(r)+string(c))
			}
		}
		ret = tmp
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      []string
	}{
		{
			description: "testing 1",
			input:       "23",
			expect:      []string{"ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"},
		},
	}

	for _, tt := range testCases {
		actual := letterCombinations(tt.input)
		for _, ex := range tt.expect {
			found := false
			for _, ac := range actual {
				if ex == ac {
					found = true
					break
				}
			}
			if !found {
				log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
			}
		}
	}
}
