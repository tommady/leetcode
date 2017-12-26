package main

import (
	"log"
	"reflect"
)

// Reverse a singly linked list.
// Hint:
// A linked list can be reversed either iteratively or recursively. Could you implement both?

// ListNode defining for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var ret *ListNode

	for head != nil {
		next := head.Next
		head.Next = ret
		ret = head
		head = next
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       *ListNode
		expect      *ListNode
	}{
		{
			description: "testing 1",
			input:       genList([]int{1, 2, 3}),
			expect:      genList([]int{3, 2, 1}),
		},
	}

	for _, tt := range testCases {
		actual := reverseList(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}

func genList(nums []int) *ListNode {
	if len(nums) == 0 {
		return nil
	}

	root := &ListNode{Val: nums[0]}
	looper := root

	for i := 1; i < len(nums); i++ {
		looper.Next = new(ListNode)
		looper = looper.Next
		looper.Val = nums[i]
	}

	return root
}
