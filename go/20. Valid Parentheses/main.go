package main

import "log"

type stack struct {
	cache []byte
}

func (s *stack) push(b byte) {
	s.cache = append(s.cache, b)
}

func (s *stack) pop() byte {
	l := len(s.cache)
	if l == 0 {
		return ' '
	}
	ret := s.cache[l-1]
	s.cache = s.cache[:l-1]
	return ret
}

func (s *stack) len() int {
	return len(s.cache)
}

func isValid(s string) bool {
	length := len(s)
	if length <= 1 {
		return false
	}

	stac := &stack{}

	for i := 0; i < length; i++ {
		match := byte(' ')
		switch s[i] {
		case '}':
			match = '{'
		case ']':
			match = '['
		case ')':
			match = '('
		default:
			stac.push(s[i])
		}

		if match != ' ' && match != stac.pop() {
			return false
		}
	}

	if stac.len() > 0 {
		return false
	}
	return true
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      bool
	}{
		{
			description: "testing 1",
			input:       "()[]{}",
			expect:      true,
		},
		{
			description: "testing 2",
			input:       "([)]",
			expect:      false,
		},
		{
			description: "testing 3",
			input:       "([])",
			expect:      true,
		},
		{
			description: "testing 4",
			input:       "((",
			expect:      false,
		},
		{
			description: "testing 5",
			input:       "([]",
			expect:      false,
		},
	}

	for _, tt := range testCases {
		actual := isValid(tt.input)
		if tt.expect != actual {
			log.Fatalf(
				"%s: input:%v, expect[%v] != actual[%v]",
				tt.description,
				tt.input,
				tt.expect,
				actual,
			)
		}
	}
}
