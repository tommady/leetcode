package main

import (
	"log"
	"math"
)

// Given a binary tree, determine if it is a valid binary search tree (BST).

// Assume a BST is defined as follows:

// The left subtree of a node contains only nodes with keys less than the node's key.
// The right subtree of a node contains only nodes with keys greater than the node's key.
// Both the left and right subtrees must also be binary search trees.
// Example 1:
//     2
//    / \
//   1   3
// Binary tree [2,1,3], return true.
// Example 2:
//     1
//    / \
//   2   3
// Binary tree [1,2,3], return false.

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	return bst(root, math.MinInt64, math.MaxInt64)
}

func bst(root *TreeNode, min, max int) bool {
	if root == nil {
		return true
	}

	if root.Val >= max || root.Val <= min {
		return false
	}

	return bst(root.Left, min, root.Val) && bst(root.Right, root.Val, max)
}

func main() {
	testCases := []struct {
		description string
		input       *TreeNode
		expect      bool
	}{
		{
			description: "testing 1",
			input:       &TreeNode{Val: 2, Left: &TreeNode{Val: 1}, Right: &TreeNode{Val: 3}},
			expect:      true,
		},
		{
			description: "testing 2",
			input:       &TreeNode{Val: 1, Left: &TreeNode{Val: 2}, Right: &TreeNode{Val: 3}},
			expect:      false,
		},
		{
			description: "testing 3",
			input:       &TreeNode{Val: 1, Left: &TreeNode{Val: 1}},
			expect:      false,
		},
		{
			description: "testing 4",
			input: &TreeNode{
				Val: 10,
				Left: &TreeNode{
					Val: 5,
				},
				Right: &TreeNode{
					Val: 15,
					Left: &TreeNode{
						Val: 6,
					},
					Right: &TreeNode{
						Val: 20,
					},
				},
			},
			expect: false,
		},
	}

	for _, tt := range testCases {
		actual := isValidBST(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
