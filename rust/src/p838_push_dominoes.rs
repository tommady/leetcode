// There are N dominoes in a line, and we place each domino vertically upright.  In the beginning, we simultaneously push some of the dominoes either to the left or to the right.    After each second, each domino that is falling to the left pushes the adjacent domino on the left.  Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.  When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.  For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.  Given a string S representing the initial state. S[i] = &#39;L&#39;, if the i-th domino has been pushed to the left; S[i] = &#39;R&#39;, if the i-th domino has been pushed to the right; S[i] = &#39;.&#39;, if the i-th domino has not been pushed.  Return a string representing the final state.   Example 1:   Input: .L.R...LR..L..  Output: LL.RR.LLRRLL..    Example 2:   Input: RR.L  Output: RR.L  Explanation: The first domino expends no additional force on the second domino.   Note:   0 = N = 10^5 String dominoes contains only &#39;L&#39;, &#39;R&#39; and &#39;.&#39;
//
//
// ".L.R...LR..L.."
// "RR.L"
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn push_dominoes(mut dominoes: String) -> String {
        if dominoes.len() <= 1 {
            return dominoes;
        }

        dominoes = format!("L{}R", dominoes);

        let mut ret: Vec<char> = dominoes.chars().collect();
        let mut i = 0;
        let mut j = 1;
        let bytes = dominoes.as_bytes();

        while i < dominoes.len() && j < dominoes.len() {
            while bytes[j] == b'.' {
                j += 1;
            }

            if bytes[i] == b'L' && bytes[j] == b'L' {
                let mut fill = i + 1;
                while fill < j {
                    ret[fill] = 'L';
                    fill += 1;
                }
            } else if bytes[i] == b'R' && bytes[j] == b'R' {
                let mut fill = i + 1;
                while fill < j {
                    ret[fill] = 'R';
                    fill += 1;
                }
            } else if bytes[i] == b'R' && bytes[j] == b'L' {
                let mut r = j - 1;
                let mut l = i + 1;
                while l < r {
                    ret[l] = 'R';
                    ret[r] = 'L';
                    r -= 1;
                    l += 1;
                }
            }

            i = j;
            j += 1;
        }

        ret.drain(0..1);
        ret.drain(ret.len() - 1..ret.len());
        ret.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_838_solution() {
        assert_eq!(
            "LL.RR.LLRRLL..".to_string(),
            Solution::push_dominoes(".L.R...LR..L..".to_string()),
        );
        assert_eq!(
            "RR.L".to_string(),
            Solution::push_dominoes("RR.L".to_string()),
        );
        assert_eq!(
            "..RRR".to_string(),
            Solution::push_dominoes("..R..".to_string()),
        );
    }
}
