package main

import "log"

func containsDuplicate(nums []int) bool {
	m := make(map[int]struct{})
	for _, n := range nums {
		if _, exist := m[n]; exist {
			return true
		}
		m[n] = struct{}{}
	}

	return false
}

func main() {
	testCases := []struct {
		description string
		input       []int
		expect      bool
	}{
		{
			description: "testing 1",
			input:       []int{1, 2, 3},
			expect:      false,
		},
		{
			description: "testing 2",
			input:       []int{1, 1, 3},
			expect:      true,
		},
	}

	for _, tt := range testCases {
		actual := containsDuplicate(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
