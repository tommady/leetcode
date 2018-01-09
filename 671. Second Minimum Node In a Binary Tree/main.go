package main

import "log"
import "math"

// Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes.

// Given such a binary tree, you need to output the second minimum value in the set made of all the nodes' value in the whole tree.

// If no such second minimum value exists, output -1 instead.

// Example 1:
// Input:
//     2
//    / \
//   2   5
//      / \
//     5   7

// Output: 5
// Explanation: The smallest value is 2, the second smallest value is 5.
// Example 2:
// Input:
//     2
//    / \
//   2   2

// Output: -1
// Explanation: The smallest value is 2, but there isn't any second smallest value.

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func finder(root *TreeNode, first, second *int) {
	if root == nil {
		return
	}

	if root.Val < *first {
		*first, *second = root.Val, *first
	} else if root.Val < *second && root.Val != *first {
		*second = root.Val
	}

	finder(root.Left, first, second)
	finder(root.Right, first, second)
}

func findSecondMinimumValue(root *TreeNode) int {
	first := math.MaxInt32
	second := math.MaxInt32
	finder(root, &first, &second)

	if first == second || second == math.MaxInt32 {
		return -1
	}
	return second
}

func main() {
	testCases := []struct {
		description string
		input       *TreeNode
		expect      int
	}{
		{
			description: "testing 1",
			input: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 2,
				},
				Right: &TreeNode{
					Val: 5,
					Left: &TreeNode{
						Val: 5,
					},
					Right: &TreeNode{
						Val: 7,
					},
				},
			},
			expect: 5,
		},
		{
			description: "testing 2",
			input: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 2,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
			expect: -1,
		},
		{
			description: "testing 3",
			input: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 2,
				},
				Right: &TreeNode{
					Val: 5,
					Left: &TreeNode{
						Val: 5,
					},
					Right: &TreeNode{
						Val: 5,
					},
				},
			},
			expect: 5,
		},
	}

	for _, tt := range testCases {
		actual := findSecondMinimumValue(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
