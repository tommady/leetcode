package main

import (
	"log"
	"reflect"
)

// Given a sorted linked list, delete all duplicates such that each element appear only once.

// For example,
// Given 1->1->2, return 1->2.
// Given 1->1->2->3->3, return 1->2->3.

// ListNode definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	cur := head
	for cur.Next != nil {
		if cur.Val == cur.Next.Val {
			cur.Next = cur.Next.Next
		} else {
			cur = cur.Next
		}
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
			input:       &ListNode{Val: 1, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2}}},
			expect:      &ListNode{Val: 1, Next: &ListNode{Val: 2}},
		},
		{
			description: "testing 2",
			input:       &ListNode{Val: 1, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 3}}}}},
			expect:      &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3}}},
		},
		{
			description: "testing 3",
			input:       &ListNode{Val: 1, Next: &ListNode{Val: 1, Next: &ListNode{Val: 1}}},
			expect:      &ListNode{Val: 1},
		},
	}

	for _, tt := range testCases {
		actual := deleteDuplicates(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}
