package main

import (
	"log"
	"reflect"
)

// Invert a binary tree.
//      4
//    /   \
//   2     7
//  / \   / \
// 1   3 6   9
// to
//      4
//    /   \
//   7     2
//  / \   / \
// 9   6 3   1
//
// Trivia:
// This problem was inspired by this original tweet by Max Howell:
// Google: 90% of our engineers use the software you wrote (Homebrew), but you can’t invert a binary tree on a whiteboard so fuck off.

// TreeNode definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func reverse(node *TreeNode) {
	if node == nil {
		return
	}

	tmp := node.Left
	node.Left = node.Right
	node.Right = tmp

	reverse(node.Left)
	reverse(node.Right)
}

func invertTree(root *TreeNode) *TreeNode {
	reverse(root)
	return root
}

func main() {
	testCases := []struct {
		description string
		input       *TreeNode
		expect      *TreeNode
	}{
		{
			description: "testing 1",
			input: &TreeNode{
				Val: 4,
				Left: &TreeNode{
					Val: 2,
					Left: &TreeNode{
						Val: 1,
					},
					Right: &TreeNode{
						Val: 3,
					},
				},
				Right: &TreeNode{
					Val: 7,
					Left: &TreeNode{
						Val: 6,
					},
					Right: &TreeNode{
						Val: 9,
					},
				},
			},
			expect: &TreeNode{
				Val: 4,
				Left: &TreeNode{
					Val: 7,
					Left: &TreeNode{
						Val: 9,
					},
					Right: &TreeNode{
						Val: 6,
					},
				},
				Right: &TreeNode{
					Val: 2,
					Left: &TreeNode{
						Val: 3,
					},
					Right: &TreeNode{
						Val: 1,
					},
				},
			},
		},
	}

	for _, tt := range testCases {
		actual := invertTree(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}
