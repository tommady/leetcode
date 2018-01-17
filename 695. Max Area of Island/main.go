package main

import "log"

// Given a non-empty 2D array grid of 0's and 1's, an island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

// Find the maximum area of an island in the given 2D array. (If there is no island, the maximum area is 0.)

// Example 1:
// [[0,0,1,0,0,0,0,1,0,0,0,0,0],
//  [0,0,0,0,0,0,0,1,1,1,0,0,0],
//  [0,1,1,0,1,0,0,0,0,0,0,0,0],
//  [0,1,0,0,1,1,0,0,1,0,1,0,0],
//  [0,1,0,0,1,1,0,0,1,1,1,0,0],
//  [0,0,0,0,0,0,0,0,0,0,1,0,0],
//  [0,0,0,0,0,0,0,1,1,1,0,0,0],
//  [0,0,0,0,0,0,0,1,1,0,0,0,0]]
// Given the above grid, return 6. Note the answer is not 11, because the island must be connected 4-directionally.
// Example 2:
// [[0,0,0,0,0,0,0,0]]
// Given the above grid, return 0.
// Note: The length of each dimension in the given grid does not exceed 50.

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func dfs(grid [][]int, i, j int) int {
	if i >= 0 && i < len(grid) && j >= 0 && j < len(grid[0]) && grid[i][j] == 1 {
		grid[i][j] = 0
		return 1 + dfs(grid, i+1, j) + dfs(grid, i-1, j) + dfs(grid, i, j-1) + dfs(grid, i, j+1)
	}

	return 0
}

func maxAreaOfIsland(grid [][]int) int {
	ret := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 1 {
				ret = max(ret, dfs(grid, i, j))
			}
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       [][]int
		expect      int
	}{
		{
			description: "testing 1",
			input: [][]int{
				[]int{0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0},
				[]int{0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
				[]int{0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
				[]int{0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0},
				[]int{0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0},
				[]int{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0},
				[]int{0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
				[]int{0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0},
			},
			expect: 6,
		},
		{
			description: "testing 2",
			input: [][]int{
				[]int{0, 0, 0, 0, 0, 0, 0, 0},
			},
			expect: 0,
		},
	}

	for _, tt := range testCases {
		actual := maxAreaOfIsland(tt.input)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%d != actual:%d", tt.description, tt.expect, actual)
		}
	}
}
