package main

import (
	"log"
	"reflect"
)

// In MATLAB, there is a very useful function called 'reshape', which can reshape a matrix into a new one with different size but keep its original data.

// You're given a matrix represented by a two-dimensional array, and two positive integers r and c representing the row number and column number of the wanted reshaped matrix, respectively.

// The reshaped matrix need to be filled with all the elements of the original matrix in the same row-traversing order as they were.

// If the 'reshape' operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.

// Example 1:
// Input:
// nums =
// [[1,2],
//  [3,4]]
// r = 1, c = 4
// Output:
// [[1,2,3,4]]
// Explanation:
// The row-traversing of nums is [1,2,3,4]. The new reshaped matrix is a 1 * 4 matrix, fill it row by row by using the previous list.
// Example 2:
// Input:
// nums =
// [[1,2],
//  [3,4]]
// r = 2, c = 4
// Output:
// [[1,2],
//  [3,4]]
// Explanation:
// There is no way to reshape a 2 * 2 matrix to a 2 * 4 matrix. So output the original matrix.
// Note:
// The height and width of the given matrix is in range [1, 100].
// The given r and c are all positive.

type pumper struct {
	nums   [][]int
	row    int
	col    int
	curRow int
	curCol int
}

func newPumper(nums [][]int) *pumper {
	return &pumper{
		nums:   nums,
		row:    len(nums) - 1,
		col:    len(nums[0]) - 1,
		curCol: -1,
	}
}

func (p *pumper) pumping() int {
	if p.curCol+1 > p.col {
		p.curCol = 0
		p.curRow++
	} else {
		p.curCol++
	}
	return p.nums[p.curRow][p.curCol]
}

type builder struct {
	nums   [][]int
	row    int
	col    int
	curRow int
	curCol int
}

func newBuilder(row, col int) *builder {
	ns := make([][]int, row)
	ns[0] = make([]int, col)
	return &builder{
		nums:   ns,
		row:    row - 1,
		col:    col - 1,
		curCol: -1,
	}
}

func (b *builder) set(v int) {
	if b.curCol+1 > b.col {
		b.curCol = 0
		b.curRow++
		b.nums[b.curRow] = make([]int, b.col+1)
	} else {
		b.curCol++
	}
	b.nums[b.curRow][b.curCol] = v
}

func (b *builder) getNums() [][]int {
	return b.nums
}

func matrixReshape(nums [][]int, r int, c int) [][]int {
	total := r * c
	react := len(nums) * len(nums[0])
	if (react < total) || (r == len(nums) && c == len(nums[0])) {
		return nums
	}

	pup := newPumper(nums)
	bui := newBuilder(r, c)

	for i := 0; i < total; i++ {
		bui.set(pup.pumping())
	}

	return bui.getNums()
}

func main() {
	testCases := []struct {
		description string
		nums        [][]int
		r           int
		c           int
		expect      [][]int
	}{
		{
			description: "testing 1",
			nums:        [][]int{[]int{1, 2}, []int{3, 4}},
			r:           1,
			c:           4,
			expect:      [][]int{[]int{1, 2, 3, 4}},
		},
		{
			description: "testing 2",
			nums:        [][]int{[]int{1, 2}, []int{3, 4}},
			r:           2,
			c:           4,
			expect:      [][]int{[]int{1, 2}, []int{3, 4}},
		},
		{
			description: "testing 3",
			nums:        [][]int{[]int{1, 2}, []int{3, 4}},
			r:           4,
			c:           1,
			expect:      [][]int{[]int{1}, []int{2}, []int{3}, []int{4}},
		},
	}

	for _, tt := range testCases {
		actual := matrixReshape(tt.nums, tt.r, tt.c)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
