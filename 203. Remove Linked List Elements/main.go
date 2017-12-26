package main

import (
	"log"
	"reflect"
)

// Remove all elements from a linked list of integers that have value val.

// Example
// Given: 1 --> 2 --> 6 --> 3 --> 4 --> 5 --> 6, val = 6
// Return: 1 --> 2 --> 3 --> 4 --> 5

// ListNode defining for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements(head *ListNode, val int) *ListNode {
	if head == nil {
		return head
	}

	bulk := new(ListNode)
	bulk.Next = head
	looper := bulk.Next
	pre := bulk

	for looper != nil {
		if looper.Val == val {
			pre.Next = looper.Next
		} else {
			pre = pre.Next
		}
		looper = looper.Next
	}

	return bulk.Next
}

func main() {
	testCases := []struct {
		description string
		input1      *ListNode
		input2      int
		expect      *ListNode
	}{
		{
			description: "testing 1",
			input1:      genList([]int{1, 2, 6, 3, 4, 5, 6}),
			input2:      6,
			expect:      genList([]int{1, 2, 3, 4, 5}),
		},
		{
			description: "testing 2",
			input1:      genList([]int{1}),
			input2:      1,
			expect:      nil,
		},
	}

	for _, tt := range testCases {
		actual := removeElements(tt.input1, tt.input2)
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
