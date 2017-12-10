package main

import "log"

// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order and each of their nodes contain a single digit.
// Add the two numbers and return it as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
// Example
//
// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// Explanation: 342 + 465 = 807.

// ListNode Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	ret := new(ListNode)
	looper := ret
	isPlus := false

	for {
		if isPlus {
			looper.Val = l1.Val + l2.Val + 1
		} else {
			looper.Val = l1.Val + l2.Val
		}

		if looper.Val >= 10 {
			isPlus = true
			looper.Val %= 10
		} else {
			isPlus = false
		}

		l1 = l1.Next
		l2 = l2.Next

		if l1 == nil && l2 != nil {
			for {
				looper.Next = new(ListNode)
				looper = looper.Next
				looper.Val = l2.Val
				l2 = l2.Next
				if isPlus {
					looper.Val++
				}
				if looper.Val >= 10 {
					isPlus = true
					looper.Val %= 10
				} else {
					isPlus = false
				}
				if l2 == nil {
					if isPlus {
						looper.Next = new(ListNode)
						looper.Next.Val = 1
					}
					break
				}
			}
			break
		} else if l1 != nil && l2 == nil {
			for {
				looper.Next = new(ListNode)
				looper = looper.Next
				looper.Val = l1.Val
				l1 = l1.Next
				if isPlus {
					looper.Val++
				}
				if looper.Val >= 10 {
					isPlus = true
					looper.Val %= 10
				} else {
					isPlus = false
				}
				if l1 == nil {
					if isPlus {
						looper.Next = new(ListNode)
						looper.Next.Val = 1
					}
					break
				}
			}
			break
		} else if l1 == nil && l2 == nil {
			if isPlus {
				looper.Next = new(ListNode)
				looper.Next.Val = 1
			}
			break
		}

		looper.Next = new(ListNode)
		looper = looper.Next
	}

	return ret
}

func extractList(root *ListNode) []int {
	ret := make([]int, 0)
	for root != nil {
		ret = append(ret, root.Val)
		root = root.Next
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		l1          *ListNode
		l2          *ListNode
		expect      *ListNode
	}{
		{
			description: "testing 1",
			l1:          &ListNode{Val: 2, Next: &ListNode{Val: 4, Next: &ListNode{Val: 3}}},
			l2:          &ListNode{Val: 5, Next: &ListNode{Val: 6, Next: &ListNode{Val: 4}}},
			expect:      &ListNode{Val: 7, Next: &ListNode{Val: 0, Next: &ListNode{Val: 8}}},
		},
		{
			description: "testing 2",
			l1:          &ListNode{Val: 5},
			l2:          &ListNode{Val: 5},
			expect:      &ListNode{Val: 0, Next: &ListNode{Val: 1}},
		},
		{
			description: "testing 3",
			l1:          &ListNode{Val: 1, Next: &ListNode{Val: 8}},
			l2:          &ListNode{Val: 0},
			expect:      &ListNode{Val: 1, Next: &ListNode{Val: 8}},
		},
		{
			description: "testing 4",
			l1:          &ListNode{Val: 1},
			l2:          &ListNode{Val: 9, Next: &ListNode{Val: 9}},
			expect:      &ListNode{Val: 0, Next: &ListNode{Val: 0, Next: &ListNode{Val: 1}}},
		},
	}

	for _, testCase := range testCases {
		ac := addTwoNumbers(testCase.l1, testCase.l2)

		expect := extractList(testCase.expect)
		actual := extractList(ac)

		if len(expect) != len(actual) {
			log.Fatalf(
				"%s: expect lenof(%v) != actual lenof(%v)",
				testCase.description,
				expect,
				actual,
			)
		}

		for i := 0; i < len(expect); i++ {
			if expect[i] != actual[i] {
				log.Fatalf(
					"%s: index[%d] expect[%d] != actual[%d]",
					testCase.description,
					i,
					expect[i],
					actual[i],
				)
			}
		}
	}

	log.Printf("pass")
}
