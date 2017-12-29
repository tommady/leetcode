package main

import (
	"log"
	"reflect"
)

// Given a List of words, return the words that can be typed using letters of alphabet on only one row's of American keyboard like the image below.

// American keyboard

// Example 1:
// Input: ["Hello", "Alaska", "Dad", "Peace"]
// Output: ["Alaska", "Dad"]
// Note:
// You may use one character in the keyboard more than once.
// You may assume the input string will only contain letters of alphabet.

var (
	row1 = map[rune]struct{}{
		'q': struct{}{},
		'Q': struct{}{},
		'w': struct{}{},
		'W': struct{}{},
		'e': struct{}{},
		'E': struct{}{},
		'r': struct{}{},
		'R': struct{}{},
		't': struct{}{},
		'T': struct{}{},
		'y': struct{}{},
		'Y': struct{}{},
		'u': struct{}{},
		'U': struct{}{},
		'i': struct{}{},
		'I': struct{}{},
		'o': struct{}{},
		'O': struct{}{},
		'p': struct{}{},
		'P': struct{}{},
	}
	row2 = map[rune]struct{}{
		'a': struct{}{},
		'A': struct{}{},
		's': struct{}{},
		'S': struct{}{},
		'd': struct{}{},
		'D': struct{}{},
		'f': struct{}{},
		'F': struct{}{},
		'g': struct{}{},
		'G': struct{}{},
		'h': struct{}{},
		'H': struct{}{},
		'j': struct{}{},
		'J': struct{}{},
		'k': struct{}{},
		'K': struct{}{},
		'l': struct{}{},
		'L': struct{}{},
	}
	row3 = map[rune]struct{}{
		'z': struct{}{},
		'Z': struct{}{},
		'x': struct{}{},
		'X': struct{}{},
		'c': struct{}{},
		'C': struct{}{},
		'v': struct{}{},
		'V': struct{}{},
		'b': struct{}{},
		'B': struct{}{},
		'n': struct{}{},
		'N': struct{}{},
		'm': struct{}{},
		'M': struct{}{},
	}
)

func findWords(words []string) []string {
	ret := make([]string, 0)

	for _, word := range words {
		counter1 := 0
		counter2 := 0
		counter3 := 0
		for _, s := range word {
			if _, exist := row1[s]; exist {
				counter1++
			} else if _, exist := row2[s]; exist {
				counter2++
			} else if _, exist := row3[s]; exist {
				counter3++
			}
		}
		if (counter1 > 0 && counter2 == 0 && counter3 == 0) ||
			(counter1 == 0 && counter2 > 0 && counter3 == 0) ||
			(counter1 == 0 && counter2 == 0 && counter3 > 0) {
			ret = append(ret, word)
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       []string
		expect      []string
	}{
		{
			description: "testing 1",
			input:       []string{"Hello", "Alaska", "Dad", "Peace"},
			expect:      []string{"Alaska", "Dad"},
		},
	}

	for _, tt := range testCases {
		actual := findWords(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
