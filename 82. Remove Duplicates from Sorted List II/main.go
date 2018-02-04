package main

import (
	"log"
	"reflect"
)

// Given a sorted linked list, delete all nodes that have duplicate numbers,
// leaving only distinct numbers from the original list.

// For example,
// Given 1->2->3->3->4->4->5, return 1->2->5.
// Given 1->1->1->2->3, return 2->3.

// ListNode defining for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	fakeHead := new(ListNode)
	fakeHead.Next = head
	curr := head
	prev := fakeHead

	for curr != nil {
		for curr.Next != nil && curr.Val == curr.Next.Val {
			curr = curr.Next
		}

		if prev.Next == curr {
			prev = prev.Next
		} else {
			prev.Next = curr.Next
		}
		curr = curr.Next
	}

	return fakeHead.Next
}

func main() {
	testCases := []struct {
		description string
		input       *ListNode
		expect      *ListNode
	}{
		{
			description: "testing 1",
			input:       linkedGenerator(1, 2, 3, 3, 4, 4, 5),
			expect:      linkedGenerator(1, 2, 5),
		},
		{
			description: "testing 2",
			input:       linkedGenerator(1, 1, 1, 2, 3),
			expect:      linkedGenerator(2, 3),
		},
	}

	for _, tt := range testCases {
		actual := deleteDuplicates(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}

func linkedGenerator(ns ...int) *ListNode {
	ret := new(ListNode)
	looper := ret
	for _, n := range ns {
		looper.Val = n
		looper.Next = new(ListNode)
		looper = looper.Next
	}

	looper = nil

	return ret
}
