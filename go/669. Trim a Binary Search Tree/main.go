package main

import (
	"log"
	"reflect"
)

// Given a binary search tree and the lowest and highest boundaries as L and R,
// trim the tree so that all its elements lies in [L, R] (R >= L).
// You might need to change the root of the tree,
// so the result should return the new root of the trimmed binary search tree.

// Example 1:
// Input:
//     1
//    / \
//   0   2

//   L = 1
//   R = 2

// Output:
//     1
//       \
//        2
// Example 2:
// Input:
//     3
//    / \
//   0   4
//    \
//     2
//    /
//   1

//   L = 1
//   R = 3

// Output:
//       3
//      /
//    2
//   /
//  1

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func trimBST(root *TreeNode, L int, R int) *TreeNode {
	if root == nil {
		return nil
	}

	if root.Val < L {
		return trimBST(root.Right, L, R)
	} else if root.Val > R {
		return trimBST(root.Left, L, R)
	} else {
		root.Left = trimBST(root.Left, L, R)
		root.Right = trimBST(root.Right, L, R)
	}

	return root
}

func main() {
	testCases := []struct {
		description string
		root        *TreeNode
		L           int
		R           int
		expect      *TreeNode
	}{
		{
			description: "testing 1",
			root: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val: 0,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
			L: 1,
			R: 2,
			expect: &TreeNode{
				Val: 1,
				Right: &TreeNode{
					Val: 2,
				},
			},
		},
		{
			description: "testing 2",
			root: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 0,
					Right: &TreeNode{
						Val: 2,
						Left: &TreeNode{
							Val: 1,
						},
					},
				},
				Right: &TreeNode{
					Val: 4,
				},
			},
			L: 1,
			R: 3,
			expect: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 2,
					Left: &TreeNode{
						Val: 1,
					},
				},
			},
		},
		{
			description: "testing 3",
			root: &TreeNode{
				Val: 1,
				Right: &TreeNode{
					Val: 2,
				},
			},
			L: 2,
			R: 4,
			expect: &TreeNode{
				Val: 2,
			},
		},
	}

	for _, tt := range testCases {
		actual := trimBST(tt.root, tt.L, tt.R)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Println()
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}
