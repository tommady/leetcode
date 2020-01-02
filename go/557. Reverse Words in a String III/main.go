package main

import (
	"log"
)

// Given a string, you need to reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

// Example 1:
// Input: "Let's take LeetCode contest"
// Output: "s'teL ekat edoCteeL tsetnoc"
// Note: In the string, each word is separated by single space and there will not be any extra space in the string.

// func reverseWords(s string) string {
// 	ws := strings.Split(s, " ")
// 	ret := make([]string, len(ws))
// 	for i, w := range ws {
// 		rs := ""
// 		for i := len(w) - 1; i >= 0; i-- {
// 			rs += string(w[i])
// 		}
// 		ret[i] = rs
// 	}

// 	return strings.Join(ret, " ")
// }

func reverseWords(s string) string {
	ret := []byte(s + " ")

	beg := 0
	for i := 0; i <= len(s); i++ {
		if ret[i] == ' ' {
			for end := i - 1; end > beg; end-- {
				tmp := ret[beg]
				ret[beg] = ret[end]
				ret[end] = tmp
				beg++
			}
			beg = i + 1
		}
	}

	return string(ret[:len(ret)-1])
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      string
	}{
		{
			description: "testing 1",
			input:       "Let's take LeetCode contest",
			expect:      "s'teL ekat edoCteeL tsetnoc",
		},
	}

	for _, tt := range testCases {
		actual := reverseWords(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
