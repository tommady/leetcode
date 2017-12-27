package main

import "log"

// Find the sum of all left leaves in a given binary tree.

// Example:

//     3
//    / \
//   9  20
//     /  \
//    15   7

// There are two left leaves in the binary tree, with values 9 and 15 respectively. Return 24.

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumOfLeftLeaves(root *TreeNode) int {
	if root == nil {
		return 0
	}

	ret := 0
	if root.Left != nil && root.Left.Left == nil && root.Left.Right == nil {
		ret += root.Left.Val
	}

	return ret + sumOfLeftLeaves(root.Right) + sumOfLeftLeaves(root.Left)
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
			expect: 24,
		},
	}

	for _, tt := range testCases {
		actual := sumOfLeftLeaves(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
