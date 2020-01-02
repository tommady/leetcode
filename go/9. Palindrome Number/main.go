package main

import "log"

// Determine whether an integer is a palindrome. Do this without extra space.

// Some hints:
// Could negative integers be palindromes? (ie, -1)

// If you are thinking of converting the integer to string, note the restriction of using extra space.

// You could also try reversing an integer.
// However, if you have solved the problem "Reverse Integer", you know that the reversed integer might overflow.
// How would you handle such case?

// There is a more generic way of solving this problem.

func isPalindrome(x int) bool {
	reverse := 0
	y := x
	for x > 0 {
		reverse *= 10
		reverse += x % 10
		x /= 10
	}
	return reverse == y
}

func main() {
	testCases := []struct {
		description string
		input       int
		expect      bool
	}{
		{
			description: "testing 1",
			input:       53235,
			expect:      true,
		},
	}

	for _, tt := range testCases {
		actual := isPalindrome(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%v] != actual[%v]", tt.description, tt.expect, actual)
		}
	}
}
