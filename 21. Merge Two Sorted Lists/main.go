package main

import (
	"log"
	"reflect"
	"sort"
)

// Merge two sorted linked lists and return it as a new list.
// The new list should be made by splicing together the nodes of the first two lists.

// Example:

// Input: 1->2->4, 1->3->4
// Output: 1->1->2->3->4->4

// ListNode definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	nums := make([]int, 0)
	for l1 != nil {
		nums = append(nums, l1.Val)
		l1 = l1.Next
	}
	for l2 != nil {
		nums = append(nums, l2.Val)
		l2 = l2.Next
	}

	sort.Ints(nums)
	ret := new(ListNode)
	node := ret
	for _, num := range nums {
		node.Next = new(ListNode)
		node = node.Next
		node.Val = num
	}

	return ret.Next
}

func main() {
	testCases := []struct {
		description string
		input1      *ListNode
		input2      *ListNode
		expect      *ListNode
	}{
		{
			description: "testing 1",
			input1:      &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 4}}},
			input2:      &ListNode{Val: 1, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4}}},
			expect: &ListNode{
				Val: 1, Next: &ListNode{
					Val: 1, Next: &ListNode{
						Val: 2, Next: &ListNode{
							Val: 3, Next: &ListNode{
								Val: 4, Next: &ListNode{
									Val: 4,
								},
							},
						},
					},
				},
			},
		},
	}

	for _, tt := range testCases {
		actual := mergeTwoLists(tt.input1, tt.input2)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect != actual", tt.description)
		}
	}
}
