package main

import "log"
import "math"

// Given an array consisting of n integers, find the contiguous subarray of given length k that has the maximum average value. And you need to output the maximum average value.

// Example 1:
// Input: [1,12,-5,-6,50,3], k = 4
// Output: 12.75
// Explanation: Maximum average is (12-5-6+50)/4 = 51/4 = 12.75
// Note:
// 1 <= k <= n <= 30,000.
// Elements of the given array will be in the range [-10,000, 10,000].

// double findMaxAverage(vector<int>& nums, int k) {
//         double sum=0, res=INT_MIN;
//         for(int i=0;i<nums.size();i++) {
//             if(i<k) sum+=nums[i];
//             else {
//                 res=max(sum, res);
//                 sum+=nums[i]-nums[i-k];
//             }
//         }
//         res=max(sum, res);
//         return res/k;
//     }

func findMaxAverage(nums []int, k int) float64 {
	sum := 0
	max := math.MinInt64

	for i := 0; i < len(nums); i++ {
		if i < k {
			sum += nums[i]
		} else {
			if sum > max {
				max = sum
			}
			sum += nums[i] - nums[i-k]
		}
	}

	if sum > max {
		max = sum
	}

	return float64(max) / float64(k)
}

func main() {
	testCases := []struct {
		description string
		input1      []int
		input2      int
		expect      float64
	}{
		{
			description: "testing 1",
			input1:      []int{1, 12, -5, -6, 50, 3},
			input2:      4,
			expect:      12.75,
		},
	}

	for _, tt := range testCases {
		actual := findMaxAverage(tt.input1, tt.input2)
		if tt.expect != actual {
			log.Fatalf("%s: expect:%v != actual:%v", tt.description, tt.expect, actual)
		}
	}
}
