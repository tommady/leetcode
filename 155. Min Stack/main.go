package main

import (
	"log"
)

// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

// push(x) -- Push element x onto stack.
// pop() -- Removes the element on top of the stack.
// top() -- Get the top element.
// getMin() -- Retrieve the minimum element in the stack.
// Example:
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin();   --> Returns -3.
// minStack.pop();
// minStack.top();      --> Returns 0.
// minStack.getMin();   --> Returns -2.

type MinStack struct {
	stack []int
	min   int
}

/** initialize your data structure here. */
func Constructor() MinStack {
	return MinStack{
		stack: make([]int, 0),
		min:   0,
	}
}

func (m *MinStack) Push(x int) {
	if len(m.stack) == 0 {
		m.stack = append(m.stack, 0)
		m.min = x
	} else {
		m.stack = append(m.stack, x-m.min)
		if x < m.min {
			m.min = x
		}
	}
}

func (m *MinStack) Pop() {
	if len(m.stack) == 0 {
		return
	}

	n := m.stack[len(m.stack)-1]
	m.stack = m.stack[:len(m.stack)-1]

	if n < 0 {
		m.min -= n
	}
}

func (m *MinStack) Top() int {
	n := m.stack[len(m.stack)-1]
	if n > 0 {
		return n + m.min
	}
	return m.min
}

func (m *MinStack) GetMin() int {
	return m.min
}

/**
 * Your MinStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.GetMin();
 */

func main() {
	obj := Constructor()
	obj.Push(-2)
	obj.Push(0)
	obj.Push(-3)

	expect := -3
	actual := obj.GetMin()
	if expect != actual {
		log.Fatalf("get min expect:%v actual:%v", expect, actual)
	}

	obj.Pop()
	expect = 0
	actual = obj.Top()
	if expect != actual {
		log.Fatalf("top expect:%v, actual:%v", expect, actual)
	}

	expect = -2
	actual = obj.GetMin()
	if expect != actual {
		log.Fatalf("get min expect:%v actual:%v", expect, actual)
	}
}
