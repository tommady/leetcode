package main

import "log"

// Given two binary trees, write a function to check if they are the same or not.

// Two binary trees are considered the same if they are structurally identical and the nodes have the same value.

// Example 1:

// Input:     1         1
//           / \       / \
//          2   3     2   3

//         [1,2,3],   [1,2,3]

// Output: true
// Example 2:

// Input:     1         1
//           /           \
//          2             2

//         [1,2],     [1,null,2]

// Output: false
// Example 3:

// Input:     1         1
//           / \       / \
//          2   1     1   2

//         [1,2,1],   [1,1,2]

// Output: false

// TreeNode defining for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
	}
	if p.Val != q.Val {
		return false
	}

	if isSameTree(p.Left, q.Left) {
		return isSameTree(p.Right, q.Right)
	}

	return false
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
			input1:      &TreeNode{Val: 1, Left: &TreeNode{Val: 2}},
			input2:      &TreeNode{Val: 1, Right: &TreeNode{Val: 2}},
			expect:      false,
		},
		{
			description: "testing 2",
			input1:      &TreeNode{Val: 1, Left: &TreeNode{Val: 2}, Right: &TreeNode{Val: 1}},
			input2:      &TreeNode{Val: 1, Right: &TreeNode{Val: 2}, Left: &TreeNode{Val: 1}},
			expect:      false,
		},
		{
			description: "testing 3",
			input1:      &TreeNode{Val: 1, Left: &TreeNode{Val: 2}, Right: &TreeNode{Val: 3}},
			input2:      &TreeNode{Val: 1, Right: &TreeNode{Val: 3}, Left: &TreeNode{Val: 2}},
			expect:      true,
		},
	}

	for _, tt := range testCases {
		actual := isSameTree(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect != actaul", tt.description)
		}
	}
}
