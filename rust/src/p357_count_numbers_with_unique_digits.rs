// Given an integer n, return the count of all numbers with unique digits, x, where 0 = x 10n. Example 1: Input: n = 2 Output: 91 Explanation: The answer should be the total numbers in the range of 0 x 100, excluding 11,22,33,44,55,66,77,88,99 Example 2: Input: n = 0 Output: 1 Constraints: 0 = n = 8
//
//
// 2
// 0
//

#![allow(dead_code)]
pub struct Solution {}

//     if n == 0 {
// 		return 1
// 	}
//
// 	ret := 10
// 	udigits := 9
// 	availableNum := 9
//
// 	for n > 1 && availableNum > 0 {
// 		udigits = udigits * availableNum
// 		ret += udigits
// 		availableNum--
// 		n--
// 	}
//
// 	return ret

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut n = n;
        let mut ret = 10;
        let mut udigits = 9;
        let mut available_num = 9;

        while n > 1 && available_num > 0 {
            udigits = udigits * available_num;
            ret += udigits;
            available_num -= 1;
            n -= 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_357_solution() {
        assert_eq!(91, Solution::count_numbers_with_unique_digits(2));
        assert_eq!(1, Solution::count_numbers_with_unique_digits(0));
    }
}
