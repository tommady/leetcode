// You are given an integer n that consists of exactly 3 digits. We call the number n fascinating if, after the following modification, the resulting number contains all the digits from 1 to 9 exactly once and does not contain any 0&#39;s: Concatenate n with the numbers 2 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n and 3 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n. Return true if n is fascinating, or false otherwise. Concatenating two numbers means joining them together. For example, the concatenation of 121 and 371 is 121371. Example 1: Input: n = 192 Output: true Explanation: We concatenate the numbers n = 192 and 2 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n = 384 and 3 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n = 576. The resulting number is 192384576. This number contains all the digits from 1 to 9 exactly once. Example 2: Input: n = 100 Output: false Explanation: We concatenate the numbers n = 100 and 2 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n = 200 and 3 Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs n = 300. The resulting number is 100200300. This number does not satisfy any of the conditions. Constraints: 100 = n = 999
//
//
// 192
// 100
// 267, 534, 801
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut m = [0; 10];

        for num in Solution::digits(n)
            .chain(Solution::digits(n * 2))
            .chain(Solution::digits(n * 3))
        {
            if num == 0 {
                return false;
            }

            let i = num as usize;
            m[i] += 1;
            if m[i] > 1 {
                return false;
            }
        }

        for num in m.iter().skip(1) {
            if *num != 1 {
                return false;
            }
        }

        true
    }

    fn digits(mut num: i32) -> impl Iterator<Item = i32> {
        let mut divisor = 1;
        while num >= divisor * 10 {
            divisor *= 10;
        }

        std::iter::from_fn(move || {
            if divisor == 0 {
                None
            } else {
                let v = num / divisor;
                num %= divisor;
                divisor /= 10;
                Some(v)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2729_solution() {
        assert!(Solution::is_fascinating(192));
        assert!(!Solution::is_fascinating(100));
        assert!(!Solution::is_fascinating(267));
    }
}
