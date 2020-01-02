package main

import "fmt"

const (
	land  = 1
	water = 0
)

func islandPerimeter(grid [][]int) int {
	perimeter := 0

	yLimit := len(grid) - 1
	for y := 0; y <= yLimit; y++ {

		xLimit := len(grid[y]) - 1
		for x := 0; x <= xLimit; x++ {
			if grid[y][x] == land {
				if x-1 >= 0 {
					if grid[y][x-1] == water {
						perimeter++
					}
				} else {
					perimeter++
				}
				if x+1 <= xLimit {
					if grid[y][x+1] == water {
						perimeter++
					}
				} else {
					perimeter++
				}
				if y-1 >= 0 {
					if grid[y-1][x] == water {
						perimeter++
					}
				} else {
					perimeter++
				}
				if y+1 <= yLimit {
					if grid[y+1][x] == water {
						perimeter++
					}
				} else {
					perimeter++
				}
			}
		}
	}
	return perimeter
}

func main() {
	expect := 16
	actual := islandPerimeter([][]int{
		{0, 1, 0, 0},
		{1, 1, 1, 0},
		{0, 1, 0, 0},
		{1, 1, 0, 0},
	})
	if expect != actual {
		fmt.Printf("expect:%v, actual:%v\n", expect, actual)
	}
}
