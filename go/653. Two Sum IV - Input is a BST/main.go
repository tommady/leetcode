package main

import "log"

// Given a Binary Search Tree and a target number,
// return true if there exist two elements in the BST such that their sum is equal to the given target.

// Example 1:
// Input:
//     5
//    / \
//   3   6
//  / \   \
// 2   4   7

// Target = 9

// Output: True
// Example 2:
// Input:
//     5
//    / \
//   3   6
//  / \   \
// 2   4   7

// Target = 28

// Output: False

// TreeNode  defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func checker(root *TreeNode, m *map[int]struct{}, k int) bool {
	if root == nil {
		return false
	}

	if _, exist := (*m)[k-root.Val]; exist {
		return true
	}

	(*m)[root.Val] = struct{}{}
	return checker(root.Left, m, k) || checker(root.Right, m, k)
}

func findTarget(root *TreeNode, k int) bool {
	m := make(map[int]struct{})

	return checker(root, &m, k)
}

func main() {
	testCases := []struct {
		description string
		input1      *TreeNode
		input2      int
		expect      bool
	}{
		{
			description: "testing 1",
			input1: &TreeNode{
				Val: 5,
				Left: &TreeNode{
					Val: 3,
					Left: &TreeNode{
						Val: 2,
					},
					Right: &TreeNode{
						Val: 4,
					},
				},
				Right: &TreeNode{
					Val: 6,
					Right: &TreeNode{
						Val: 7,
					},
				},
			},
			input2: 9,
			expect: true,
		},
		{
			description: "testing 1",
			input1: &TreeNode{
				Val: 5,
				Left: &TreeNode{
					Val: 3,
					Left: &TreeNode{
						Val: 2,
					},
					Right: &TreeNode{
						Val: 4,
					},
				},
				Right: &TreeNode{
					Val: 6,
					Right: &TreeNode{
						Val: 7,
					},
				},
			},
			input2: 28,
			expect: false,
		},
	}

	for _, tt := range testCases {
		actual := findTarget(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
