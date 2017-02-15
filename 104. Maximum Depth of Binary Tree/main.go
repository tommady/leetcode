package main

import "fmt"

// TreeNode definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxDepth(root *TreeNode) int {
	leftCounter := 0
	if root.Left != nil {
		leftCounter++
		curr := root.Left
		for curr.Left != nil {
			curr = curr.Left
			leftCounter++
		}
	}

	rightCounter := 0
	if root.Right != nil {
		rightCounter++
		curr := root.Right
		for curr.Right != nil {
			curr = curr.Right
			rightCounter++
		}
	}

	if leftCounter > rightCounter {
		return leftCounter
	}
	return rightCounter
}

func main() {
	expect := 1
	actual := maxDepth(&TreeNode{
		Val: 0,
		Left: &TreeNode{
			Val: 1,
		},
	})
	if expect != actual {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
}
