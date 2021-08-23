// Hercy wants to save money for his first car. He puts money in the Leetcode bank every day. He starts by putting in  on Monday, the first day. Every day from Tuesday to Sunday, he will put in  more than the day before. On every subsequent Monday, he will put in  more than the previous Monday. Given n, return the total amount of money he will have in the Leetcode bank at the end of the nth day. Example 1: Input: n = 4 Output: 10 Explanation: After the 4th day, the total is 1 + 2 + 3 + 4 = 10. Example 2: Input: n = 10 Output: 37 Explanation: After the 10th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37. Notice that on the 2nd Monday, Hercy only puts in . Example 3: Input: n = 20 Output: 96 Explanation: After the 20th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96. Constraints: 1 = n = 1000
//
//
// 4
// 10
// 20
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ret = 0;
        let mut day = 1;
        let mut money = 1;

        while day <= n {
            ret += money;
            money += 1;
            if day % 7 == 0 {
                money = day / 7 + 1;
            }
            day += 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1716_solution() {
        assert_eq!(10, Solution::total_money(4));
        assert_eq!(37, Solution::total_money(10));
        assert_eq!(96, Solution::total_money(20));
        assert_eq!(2800, Solution::total_money(175));
    }
}
