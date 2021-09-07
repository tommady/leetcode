// There is a forest with an unknown number of rabbits. We asked n rabbits How many rabbits have the same color as you? and collected the answers in an integer array answers where answers[i] is the answer of the ith rabbit. Given the array answers, return the minimum number of rabbits that could be in the forest. Example 1: Input: answers = [1,1,2] Output: 5 Explanation: The two rabbits that answered 1 could both be the same color, say red. The rabbit that answered 2 can&#39;t be red or the answers would be inconsistent. Say the rabbit that answered 2 was blue. Then there should be 2 other blue rabbits in the forest that didn&#39;t answer into the array. The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn&#39;t. Example 2: Input: answers = [10,10,10] Output: 11 Constraints: 1 = answers.length = 1000 0 = answers[i] 1000
//
//
// [1,1,2]
// [10,10,10]
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut map = vec![0; 1000];

        for ans in answers.iter() {
            let i = *ans as usize;
            if map[i] > 0 {
                map[i] -= 1;
            } else {
                map[i] = *ans;
                ret += ans + 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_781_solution() {
        assert_eq!(5, Solution::num_rabbits(vec![1, 1, 2]));
        assert_eq!(11, Solution::num_rabbits(vec![10, 10, 10]));
    }
}
