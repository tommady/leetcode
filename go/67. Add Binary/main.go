package main

import "log"

// Given two binary strings, return their sum (also a binary string).

// For example,
// a = "11"
// b = "1"
// Return "100".

// string addBinary(string a, string b)
//     {
//         string s = "";

//         int c = 0, i = a.size() - 1, j = b.size() - 1;
//         while(i >= 0 || j >= 0 || c == 1)
//         {
//             c += i >= 0 ? a[i --] - '0' : 0;
//             c += j >= 0 ? b[j --] - '0' : 0;
//             s = char(c % 2 + '0') + s;
//             c /= 2;
//         }

//         return s;
//     }

func addBinary(a string, b string) string {
	ret := ""
	var c byte
	i := len(a) - 1
	j := len(b) - 1

	for i >= 0 || j >= 0 || c == 1 {
		if i >= 0 {
			c += a[i] - '0'
			i--
		} else {
			c += 0
		}
		if j >= 0 {
			c += b[j] - '0'
			j--
		} else {
			c += 0
		}
		ret = string(c%2+'0') + ret
		c /= 2
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input1      string
		input2      string
		expect      string
	}{
		{
			description: "testing 1",
			input1:      "11",
			input2:      "1",
			expect:      "100",
		},
	}

	for _, tt := range testCases {
		actual := addBinary(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect[%s] != actual[%s]", tt.description, tt.expect, actual)
		}
	}
}
