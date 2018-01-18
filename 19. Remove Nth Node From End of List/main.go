package main

import (
	"log"
	"reflect"
)

// Given a linked list, remove the nth node from the end of list and return its head.

// For example,

//    Given linked list: 1->2->3->4->5, and n = 2.

//    After removing the second node from the end, the linked list becomes 1->2->3->5.
// Note:
// Given n will always be valid.
// Try to do this in one pass.

// ListNode defining for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	tmp := new(ListNode)
	tmp.Next = head
	looper, reaper := tmp, tmp

	for n >= 0 {
		looper = looper.Next
		n--
	}

	for looper != nil {
		reaper = reaper.Next
		looper = looper.Next
	}

	reaper.Next = reaper.Next.Next
	return tmp.Next
}

func main() {
	testCases := []struct {
		description string
		head        *ListNode
		n           int
		expect      *ListNode
	}{
		{
			description: "testing 1",
			head: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val: 3,
						Next: &ListNode{
							Val: 4,
							Next: &ListNode{
								Val: 5,
							},
						},
					},
				},
			},
			n: 2,
			expect: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val: 3,
						Next: &ListNode{
							Val: 5,
						},
					},
				},
			},
		},
		{
			description: "testing 2",
			head:        &ListNode{Val: 1},
			n:           1,
			expect:      nil,
		},
		{
			description: "testing 3",
			head:        &ListNode{Val: 1, Next: &ListNode{Val: 2}},
			n:           1,
			expect:      &ListNode{Val: 1},
		},
	}

	for _, tt := range testCases {
		actual := removeNthFromEnd(tt.head, tt.n)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}
