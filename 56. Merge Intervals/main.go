package main

import (
	"log"
	"reflect"
	"sort"
)

// Given a collection of intervals, merge all overlapping intervals.

// For example,
// Given [1,3],[2,6],[8,10],[15,18],
// return [1,6],[8,10],[15,18].

// Interval defining for an interval.
type Interval struct {
	Start int
	End   int
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func merge(intervals []Interval) []Interval {
	if len(intervals) == 0 || len(intervals) == 1 {
		return intervals
	}

	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i].Start < intervals[j].Start
	})

	ret := []Interval{intervals[0]}
	for i := 1; i < len(intervals); i++ {
		if ret[len(ret)-1].End < intervals[i].Start {
			ret = append(ret, intervals[i])
		} else {
			ret[len(ret)-1].End = max(ret[len(ret)-1].End, intervals[i].End)
		}
	}

	return ret
}

func main() {
	testCases := []struct {
		description string
		input       []Interval
		expect      []Interval
	}{
		{
			description: "testing 1",
			input: []Interval{
				Interval{Start: 1, End: 3},
				Interval{Start: 2, End: 6},
				Interval{Start: 8, End: 10},
				Interval{Start: 15, End: 18},
			},
			expect: []Interval{
				Interval{Start: 1, End: 6},
				Interval{Start: 8, End: 10},
				Interval{Start: 15, End: 18},
			},
		},
	}

	for _, tt := range testCases {
		actual := merge(tt.input)
		if !reflect.DeepEqual(tt.expect, actual) {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
