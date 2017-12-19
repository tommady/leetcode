package main

import "log"

// Given a roman numeral, convert it to an integer.

// Input is guaranteed to be within the range from 1 to 3999.

var m map[byte]int

func init() {
	m = make(map[byte]int)
	m['I'] = 1
	m['V'] = 5
	m['X'] = 10
	m['L'] = 50
	m['C'] = 100
	m['D'] = 500
	m['M'] = 1000
}

func romanToInt(s string) int {
	length := len(s)
	ret := m[s[length-1]]

	for i := length - 2; i >= 0; i-- {
		cur := m[s[i]]
		if cur < m[s[i+1]] {
			ret -= cur
		} else {
			ret += cur
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      int
	}{
		{
			description: "testing 1",
			input:       "VI",
			expect:      6,
		},
		{
			description: "testing 2",
			input:       "DCXXI",
			expect:      621,
		},
	}

	for _, tt := range testCases {
		actual := romanToInt(tt.input)
		if tt.expect != actual {
			log.Fatalf(
				"%s: input[%v], expect[%v] != actual[%v]",
				tt.description, tt.input, tt.expect, actual,
			)
		}
	}
}
