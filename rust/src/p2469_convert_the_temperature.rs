// You are given a non-negative floating point number rounded to two decimal places celsius, that denotes the temperature in Celsius. You should convert Celsius into Kelvin and Fahrenheit and return it as an array ans = [kelvin, fahrenheit]. Return the array ans. Answers within 10-5 of the actual answer will be accepted. Note that: Kelvin = Celsius + 273.15 Fahrenheit = Celsius Cargo.lock Cargo.toml Justfile new rustfmt.toml src target template.rs 1.80 + 32.00 Example 1: Input: celsius = 36.50 Output: [309.65000,97.70000] Explanation: Temperature at 36.50 Celsius converted in Kelvin is 309.65 and converted in Fahrenheit is 97.70. Example 2: Input: celsius = 122.11 Output: [395.26000,251.79800] Explanation: Temperature at 122.11 Celsius converted in Kelvin is 395.26 and converted in Fahrenheit is 251.798. Constraints: 0 = celsius = 1000
//
//
// 36.50
// 122.11
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.80 + 32.00]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2469_solution() {
        assert_eq!(
            vec![309.65000, 97.70000],
            Solution::convert_temperature(36.50)
        );
        assert_eq!(
            vec![395.26000, 251.79800],
            Solution::convert_temperature(122.11)
        );
    }
}
