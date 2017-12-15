package main

import (
	"log"
	"sort"
)

// Given an array of strings, group anagrams together.

// For example, given: ["eat", "tea", "tan", "ate", "nat", "bat"],
// Return:

// [
//   ["ate", "eat","tea"],
//   ["nat","tan"],
//   ["bat"]
// ]
// Note: All inputs will be in lower-case.

type sortRunes []rune

func (s sortRunes) Less(i, j int) bool {
	return s[i] < s[j]
}

func (s sortRunes) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}

func (s sortRunes) Len() int {
	return len(s)
}

func sortString(s string) string {
	r := []rune(s)
	sort.Sort(sortRunes(r))
	return string(r)
}

func groupAnagrams(strs []string) [][]string {
	m := make(map[string][]string)
	for _, str := range strs {
		sortStr := sortString(str)
		m[sortStr] = append(m[sortStr], str)
	}

	ret := make([][]string, 0)
	for _, n := range m {
		ret = append(ret, n)
	}

	return ret
}

func checkEquals(expect, actual [][]string) bool {
	if len(expect) != len(actual) {
		return false
	}
	for i := range expect {
		for j := range expect[i] {

			found := false
			for _, ac := range actual {
				for _, a := range ac {
					if a == expect[i][j] {
						found = true
					}
				}
			}
			if !found {
				return false
			}
		}
	}

	return true
}

func main() {
	testCases := []struct {
		description string
		input       []string
		expect      [][]string
	}{
		{
			description: "testing 1",
			input:       []string{"eat", "tea", "tan", "ate", "nat", "bat"},
			expect: [][]string{
				{"ate", "eat", "tea"},
				{"nat", "tan"},
				{"bat"},
			},
		},
	}

	for _, testCase := range testCases {
		actual := groupAnagrams(testCase.input)
		if !checkEquals(testCase.expect, actual) {
			log.Fatalf("%s: expect[%v] != actual[%v]", testCase.description, testCase.expect, actual)
		}
	}
}
