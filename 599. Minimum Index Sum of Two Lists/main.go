package main

import (
	"log"
	"math"
	"reflect"
)

// Suppose Andy and Doris want to choose a restaurant for dinner, and they both have a list of favorite restaurants represented by strings.

// You need to help them find out their common interest with the least list index sum. If there is a choice tie between answers, output all of them with no order requirement. You could assume there always exists an answer.

// Example 1:
// Input:
// ["Shogun", "Tapioca Express", "Burger King", "KFC"]
// ["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]
// Output: ["Shogun"]
// Explanation: The only restaurant they both like is "Shogun".
// Example 2:
// Input:
// ["Shogun", "Tapioca Express", "Burger King", "KFC"]
// ["KFC", "Shogun", "Burger King"]
// Output: ["Shogun"]
// Explanation: The restaurant they both like and have the least index sum is "Shogun" with index sum 1 (0+1).
// Note:
// 1. The length of both lists will be in the range of [1, 1000].
// 2. The length of strings in both lists will be in the range of [1, 30].
// 3. The index is starting from 0 to the list length minus 1.
// 4. No duplicates in both lists.

func findRestaurant(list1 []string, list2 []string) []string {
	lm1 := make(map[string]int)
	for index, option := range list1 {
		lm1[option] = index
	}

	indexSum := math.MaxInt64
	ret := make([]string, 0)
	for i, option := range list2 {
		if j, exist := lm1[option]; exist {
			tmpSum := i + j
			if tmpSum <= indexSum {
				if tmpSum < indexSum {
					ret = make([]string, 0)
					indexSum = tmpSum
				}
				ret = append(ret, option)
			}
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input1      []string
		input2      []string
		expect      []string
	}{
		{
			description: "testing 1",
			input1:      []string{"Shogun", "Tapioca Express", "Burger King", "KFC"},
			input2:      []string{"Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"},
			expect:      []string{"Shogun"},
		},
		{
			description: "testing 2",
			input1:      []string{"Shogun", "Tapioca Express", "Burger King", "KFC"},
			input2:      []string{"KFC", "Shogun", "Burger King"},
			expect:      []string{"Shogun"},
		},
		{
			description: "testing 3",
			input1:      []string{"Shogun", "Tapioca Express", "Burger King", "KFC"},
			input2:      []string{"KFC", "Burger King", "Tapioca Express", "Shogun"},
			expect:      []string{"KFC", "Burger King", "Tapioca Express", "Shogun"},
		},
	}

	for _, tt := range testCases {
		actual := findRestaurant(tt.input1, tt.input2)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
