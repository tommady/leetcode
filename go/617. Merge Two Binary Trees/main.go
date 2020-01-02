package main

import (
	"log"
	"reflect"
)

// Given two binary trees and imagine that when you put one of them to cover the other,
// some nodes of the two trees are overlapped while the others are not.

// You need to merge them into a new binary tree.
// The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node.
// Otherwise, the NOT null node will be used as the node of new tree.

// Example 1:
// Input:
// 	Tree 1                     Tree 2
//           1                         2
//          / \                       / \
//         3   2                     1   3
//        /                           \   \
//       5                             4   7
// Output:
// Merged tree:
// 	     3
// 	    / \
// 	   4   5
// 	  / \   \
// 	 5   4   7
// Note: The merging process must start from the root nodes of both trees.

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func merge(t1 *TreeNode, t2 *TreeNode, ret *TreeNode) {
	if t1 == nil && t2 == nil {
		return
	}

	var t1LeftNext, t2LeftNext, t1RightNext, t2RightNext *TreeNode
	var v int

	if t1 != nil {
		v += t1.Val
		t1LeftNext = t1.Left
		t1RightNext = t1.Right
	}
	if t2 != nil {
		v += t2.Val
		t2LeftNext = t2.Left
		t2RightNext = t2.Right
	}

	ret.Val = v
	if t1LeftNext != nil || t2LeftNext != nil {
		ret.Left = new(TreeNode)
	}
	if t1RightNext != nil || t2RightNext != nil {
		ret.Right = new(TreeNode)
	}

	merge(t1LeftNext, t2LeftNext, ret.Left)
	merge(t1RightNext, t2RightNext, ret.Right)
}

func mergeTrees(t1 *TreeNode, t2 *TreeNode) *TreeNode {
	if t1 == nil && t2 == nil {
		return nil
	}

	ret := new(TreeNode)
	merge(t1, t2, ret)
	return ret
}

func main() {
	testCases := []struct {
		description string
		input1      *TreeNode
		input2      *TreeNode
		expect      *TreeNode
	}{
		{
			description: "testing 1",
			input1: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val: 3,
					Left: &TreeNode{
						Val: 5,
					},
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
			input2: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 1,
					Right: &TreeNode{
						Val: 4,
					},
				},
				Right: &TreeNode{
					Val: 3,
					Right: &TreeNode{
						Val: 7,
					},
				},
			},
			expect: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 4,
					Left: &TreeNode{
						Val: 5,
					},
					Right: &TreeNode{
						Val: 4,
					},
				},
				Right: &TreeNode{
					Val: 5,
					Right: &TreeNode{
						Val: 7,
					},
				},
			},
		},
	}

	for _, tt := range testCases {
		actual := mergeTrees(tt.input1, tt.input2)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
