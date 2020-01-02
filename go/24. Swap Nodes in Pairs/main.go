package main

import (
	"log"
	"reflect"
)

// Given a linked list, swap every two adjacent nodes and return its head.

// For example,
// Given 1->2->3->4, you should return the list as 2->1->4->3.

// Your algorithm should use only constant space.
// You may not modify the values in the list, only nodes itself can be changed.

// ListNode defining for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	looper := head
	for looper != nil && looper.Next != nil {
		tmp := looper.Val
		looper.Val = looper.Next.Val
		looper.Next.Val = tmp

		looper = looper.Next.Next
	}

	return head
}

func main() {
	testCases := []struct {
		description string
		input       *ListNode
		expect      *ListNode
	}{
		{
			description: "testing 1",
			input: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val: 3,
						Next: &ListNode{
							Val: 4,
						},
					},
				},
			},
			expect: &ListNode{
				Val: 2,
				Next: &ListNode{
					Val: 1,
					Next: &ListNode{
						Val: 4,
						Next: &ListNode{
							Val: 3,
						},
					},
				},
			},
		},
		{
			description: "testing 2",
			input: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val: 3,
					},
				},
			},
			expect: &ListNode{
				Val: 2,
				Next: &ListNode{
					Val: 1,
					Next: &ListNode{
						Val: 3,
					},
				},
			},
		},
	}

	for _, tt := range testCases {
		actual := swapPairs(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}
