package main

import "log"

// Initially, there is a Robot at position (0, 0). Given a sequence of its moves,
// judge if this robot makes a circle, which means it moves back to the original place.

// The move sequence is represented by a string. And each move is represent by a character.
// The valid robot moves are R (Right), L (Left), U (Up) and D (down).
// The output should be true or false representing whether the robot makes a circle.

// Example 1:
// Input: "UD"
// Output: true
// Example 2:
// Input: "LL"
// Output: false

func judgeCircle(moves string) bool {
	pos := []int{0, 0}
	for _, m := range moves {
		switch m {
		case 'U':
			pos[1]++
		case 'D':
			pos[1]--
		case 'L':
			pos[0]--
		case 'R':
			pos[0]++
		default:
			return false
		}
	}

	return pos[0] == 0 && pos[1] == 0
}

func main() {
	testCases := []struct {
		description string
		input       string
		expect      bool
	}{
		{
			description: "testing 1",
			input:       "UD",
			expect:      true,
		},
		{
			description: "testing 2",
			input:       "LL",
			expect:      false,
		},
	}

	for _, tt := range testCases {
		actual := judgeCircle(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
