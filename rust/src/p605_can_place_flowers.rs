// Suppose you have a long flowerbed in which some of the plots are planted and some are not. However, flowers cannot be planted in adjacent plots - they would compete for water and both would die.
//
// Given a flowerbed (represented as an array containing 0 and 1, where 0 means empty and 1 means not empty), and a number n, return if n new flowers can be planted in it without violating the no-adjacent-flowers rule.
//
// Example 1:
//
// Input: flowerbed = [1,0,0,0,1], n = 1
// Output: True
//
//
//
// Example 2:
//
// Input: flowerbed = [1,0,0,0,1], n = 2
// Output: False
//
//
//
// Note:
//
// The input array won't violate no-adjacent-flowers rule.
// The input array size is in the range of [1, 20000].
// n is a non-negative integer which won't exceed the input array size.
//
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut remain = n;
        let mut flowers = flowerbed;

        if flowers.len() == 1 {
            if flowers[0] == 0 {
                remain -= 1;
            }
        } else {
            if flowers[0] == 0 && flowers[1] == 0 {
                flowers[0] = 1;
                remain -= 1;
            }

            let len = flowers.len() - 1;
            if flowers[len] == 0 && flowers[len - 1] == 0 {
                flowers[len] = 1;
                remain -= 1;
            }

            for i in 1..flowers.len() - 1 {
                if flowers[i] == 0 && flowers[i - 1] == 0 && flowers[i + 1] == 0 {
                    remain -= 1;
                    flowers[i] = 1;
                }
            }
        }

        remain <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_605_solution() {
        assert!(
            Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1),
            "vec = [1,0,0,0,1], n = 1"
        );
        assert!(
            !Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2),
            "vec = [1,0,0,0,1], n = 2"
        );
        assert!(
            !Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
            "vec = [1,0,0,0,0,1], n = 2"
        );
    }
}
