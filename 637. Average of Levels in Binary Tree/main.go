package main

import (
	"log"
	"reflect"
)

// Given a non-empty binary tree, return the average value of the nodes on each level in the form of an array.
// Example 1:
// Input:
//     3
//    / \
//   9  20
//     /  \
//    15   7
// Output: [3, 14.5, 11]
// Explanation:
// The average value of nodes on level 0 is 3,  on level 1 is 14.5, and on level 2 is 11. Hence return [3, 14.5, 11].
// Note:
// The range of node's value is in the range of 32-bit signed integer.

// TreeNode Defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func counter(root *TreeNode, sum, many *[]int, level int) {
	if root == nil {
		return
	}

	if len(*sum) <= level {
		*sum = append(*sum, root.Val)
	} else {
		(*sum)[level] += root.Val
	}
	if len(*many) <= level {
		*many = append(*many, 1)
	} else {
		(*many)[level]++
	}

	counter(root.Left, sum, many, level+1)
	counter(root.Right, sum, many, level+1)
}

func averageOfLevels(root *TreeNode) []float64 {
	sum := make([]int, 0)
	many := make([]int, 0)
	counter(root, &sum, &many, 0)

	ret := make([]float64, len(sum))
	for i := 0; i < len(many); i++ {
		ret[i] = float64(sum[i]) / float64(many[i])
	}
	return ret
}

func main() {
	testCases := []struct {
		description string
		input       *TreeNode
		expect      []float64
	}{
		{
			description: "testing 1",
			input: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 9,
				},
				Right: &TreeNode{
					Val: 20,
					Left: &TreeNode{
						Val: 15,
					},
					Right: &TreeNode{
						Val: 7,
					},
				},
			},
			expect: []float64{3, 14.5, 11},
		},
	}

	for _, tt := range testCases {
		actual := averageOfLevels(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
