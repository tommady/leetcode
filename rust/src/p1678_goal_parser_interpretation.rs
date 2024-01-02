// You own a Goal Parser that can interpret a string command. The command consists of an alphabet of G , () and/or (al) in some order. The Goal Parser will interpret G as the string G , () as the string o , and (al) as the string al . The interpreted strings are then concatenated in the original order. Given the string command, return the Goal Parser&#39;s interpretation of command. Example 1: Input: command = G()(al) Output: Goal Explanation: The Goal Parser interprets the command as follows: G - G () - o (al) - al The final concatenated result is Goal . Example 2: Input: command = G()()()()(al) Output: Gooooal Example 3: Input: command = (al)G(al)()()G Output: alGalooG Constraints: 1 = command.length = 100 command consists of G , () , and/or (al) in some order.
//
//
// "G()(al)"
// "G()()()()(al)"
// "(al)G(al)()()G"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ret = String::new();

        let mut found_bracket_upper = false;
        let mut found_a = false;
        let mut found_l = false;

        for c in command.chars() {
            match c {
                '(' => found_bracket_upper = true,
                ')' => {
                    if found_l {
                        ret.push_str("al")
                    } else {
                        ret.push_str("o")
                    }
                    found_bracket_upper = false;
                    found_a = false;
                    found_l = false;
                }
                'a' => {
                    if found_bracket_upper {
                        found_a = true
                    }
                }
                'l' => {
                    if found_a {
                        found_l = true;
                    }
                }
                'G' => ret.push_str("G"),
                _ => panic!("should not be here"),
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1678_solution() {
        assert_eq!(
            "Goal".to_string(),
            Solution::interpret("G()(al)".to_string())
        );

        assert_eq!(
            "Gooooal".to_string(),
            Solution::interpret("G()()()()(al)".to_string())
        );

        assert_eq!(
            "alGalooG".to_string(),
            Solution::interpret("(al)G(al)()()G".to_string())
        );
    }
}
