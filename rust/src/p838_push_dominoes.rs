// There are N dominoes in a line, and we place each domino vertically upright.
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