package main

import (
	"log"
	"reflect"
)

// Reverse a linked list from position m to n. Do it in-place and in one-pass.

// For example:
// Given 1->2->3->4->5->NULL, m = 2 and n = 4,

// return 1->4->3->2->5->NULL.

// Note:
// Given m, n satisfy the following condition:
// 1 ≤ m ≤ n ≤ length of list.

// ListNode defining for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseBetween(head *ListNode, m int, n int) *ListNode {
	if head == nil {
		return nil
	}

	fakeHead := new(ListNode)
	fakeHead.Next = head
	looper := fakeHead
	for i := 0; i < m-1; i++ {
		looper = looper.Next
	}

	beg := looper.Next
	end := beg.Next
	for i := 0; i < n-m; i++ {
		beg.Next = end.Next
		end.Next = looper.Next
		looper.Next = end
		end = beg.Next
	}

	return fakeHead.Next
}

func main() {
	testCases := []struct {
		description string
		head        *ListNode
		m           int
		n           int
		expect      *ListNode
	}{
		{
			description: "testing 1",
			head:        genList(1, 2, 3, 4, 5),
			m:           2,
			n:           4,
			expect:      genList(1, 4, 3, 2, 5),
		},
	}

	for _, tt := range testCases {
		actual := reverseBetween(tt.head, tt.m, tt.n)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}

func genList(ns ...int) *ListNode {
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
