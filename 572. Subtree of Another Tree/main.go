package main

import "log"

// Given two non-empty binary trees s and t,
// check whether tree t has exactly the same structure and node values with a subtree of s.
// A subtree of s is a tree consists of a node in s and all of this node's descendants.
// The tree s could also be considered as a subtree of itself.

// Example 1:
// Given tree s:

//      3
//     / \
//    4   5
//   / \
//  1   2
// Given tree t:
//    4
//   / \
//  1   2
// Return true, because t has the same structure and node values with a subtree of s.
// Example 2:
// Given tree s:

//      3
//     / \
//    4   5
//   / \
//  1   2
//     /
//    0
// Given tree t:
//    4
//   / \
//  1   2
// Return false.

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isTheSame(s *TreeNode, t *TreeNode) bool {
	if s == nil {
		return t == nil
	}
	if t == nil {
		return s == nil
	}
	if s.Val != t.Val {
		return false
	}
	return isTheSame(s.Left, t.Left) && isTheSame(s.Right, t.Right)
}

func isSubtree(s *TreeNode, t *TreeNode) bool {
	if s == nil {
		return false
	}
	if isTheSame(s, t) {
		return true
	}
	return isSubtree(s.Left, t) || isSubtree(s.Right, t)
}

func main() {
	testCases := []struct {
		description string
		input1      *TreeNode
		input2      *TreeNode
		expect      bool
	}{
		{
			description: "testing 1",
			input1: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 4,
					Left: &TreeNode{
						Val: 1,
					},
					Right: &TreeNode{
						Val: 2,
					},
				},
				Right: &TreeNode{
					Val: 5,
				},
			},
			input2: &TreeNode{
				Val: 4,
				Left: &TreeNode{
					Val: 1,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
			expect: true,
		},
		{
			description: "testing 2",
			input1: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 4,
					Left: &TreeNode{
						Val: 1,
					},
					Right: &TreeNode{
						Val: 2,
						Left: &TreeNode{
							Val: 0,
						},
					},
				},
				Right: &TreeNode{
					Val: 5,
				},
			},
			input2: &TreeNode{
				Val: 4,
				Left: &TreeNode{
					Val: 1,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
			expect: false,
		},
		{
			description: "testing 3",
			input1: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val: 1,
				},
			},
			input2: &TreeNode{
				Val: 1,
			},
			expect: true,
		},
	}

	for _, tt := range testCases {
		actual := isSubtree(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
