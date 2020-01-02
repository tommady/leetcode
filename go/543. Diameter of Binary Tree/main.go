package main

import "log"

// Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

// Example:
// Given a binary tree
//           1
//          / \
//         2   3
//        / \
//       4   5
// Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].

// Note: The length of path between two nodes is represented by the number of edges between them.

// TreeNode Defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func diameterOfBinaryTree(root *TreeNode) int {
	maxValue := 0
	dep(root, &maxValue)
	return maxValue
}

func dep(root *TreeNode, maxValue *int) int {
	if root == nil {
		return 0
	}

	l := dep(root.Left, maxValue)
	r := dep(root.Right, maxValue)

	*maxValue = max(*maxValue, l+r)

	return max(l, r) + 1
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
				Val: 1,
				Left: &TreeNode{
					Val: 2,
					Left: &TreeNode{
						Val: 4,
					},
					Right: &TreeNode{
						Val: 5,
					},
				},
				Right: &TreeNode{
					Val: 3,
				},
			},
			expect: 3,
		},
		{
			description: "testing 2",
			input: &TreeNode{
				Val: 1,
			},
			expect: 0,
		},
	}

	for _, tt := range testCases {
		actual := diameterOfBinaryTree(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
