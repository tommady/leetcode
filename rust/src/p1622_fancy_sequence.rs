// Write an API that generates fancy sequences using the append, addAll, and multAll operations. Implement the Fancy class: Fancy() Initializes the object with an empty sequence. void append(val) Appends an integer val to the end of the sequence. void addAll(inc) Increments all existing values in the sequence by an integer inc. void multAll(m) Multiplies all existing values in the sequence by an integer m. int getIndex(idx) Gets the current value at index idx (0-indexed) of the sequence modulo 109 + 7. If the index is greater or equal than the length of the sequence, return -1. Example 1: Input [ Fancy , append , addAll , append , multAll , getIndex , addAll , append , multAll , getIndex , getIndex , getIndex ] [[], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]] Output [null, null, null, null, null, 10, null, null, null, 26, 34, 20] Explanation Fancy fancy = new Fancy(); fancy.append(2); // fancy sequence: [2] fancy.addAll(3); // fancy sequence: [2+3] - [5] fancy.append(7); // fancy sequence: [5, 7] fancy.multAll(2); // fancy sequence: [5*2, 7*2] - [10, 14] fancy.getIndex(0); // return 10 fancy.addAll(3); // fancy sequence: [10+3, 14+3] - [13, 17] fancy.append(10); // fancy sequence: [13, 17, 10] fancy.multAll(2); // fancy sequence: [13*2, 17*2, 10*2] - [26, 34, 20] fancy.getIndex(0); // return 26 fancy.getIndex(1); // return 34 fancy.getIndex(2); // return 20 Constraints: 1 = val, inc, m = 100 0 = idx = 105 At most 105 calls total will be made to append, addAll, multAll, and getIndex.
//
//
// ["Fancy","append","addAll","append","multAll","getIndex","addAll","append","multAll","getIndex","getIndex","getIndex"]
// [[],[2],[3],[7],[2],[0],[3],[10],[2],[0],[1],[2]]
//

#![allow(dead_code)]
pub struct Solution {}

const MOD: u64 = 1_000_000_007;

struct Fancy {
    sequences: Vec<i32>,
}

/**
`&self` means the method takes an immutable reference.
If you need a mutable reference, change it to `&mut self` instead.
*/
impl Fancy {
    fn new() -> Self {
        Fancy {
            sequences: Vec::new(),
        }
    }

    fn append(&mut self, val: i32) {
        self.sequences.push(val);
    }

    fn add_all(&mut self, inc: i32) {
        for sequence in self.sequences.iter_mut() {
            *sequence = match sequence.checked_add(inc) {
                Some(v) => v,
                None => {
                    let tmp: u64 = *sequence as u64;
                    ((tmp + inc as u64) % MOD) as i32
                }
            };
        }
    }

    fn mult_all(&mut self, m: i32) {
        for sequence in self.sequences.iter_mut() {
            *sequence = match sequence.checked_mul(m) {
                Some(v) => v,
                None => {
                    let tmp: u64 = *sequence as u64;
                    ((tmp * m as u64) % MOD) as i32
                }
            };
        }
    }

    fn get_index(&self, idx: i32) -> i32 {
        if let Some(v) = self.sequences.get(idx as usize) {
            return (*v as u64 % MOD) as i32;
        }
        -1
    }
}

/**
Your Fancy object will be instantiated and called as such:
let obj = Fancy::new();
obj.append(val);
obj.add_all(inc);
obj.mult_all(m);
let ret_4: i32 = obj.get_index(idx);
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1622_solution() {
        let mut obj = Fancy::new();
        obj.append(2);
        obj.add_all(3);
        obj.append(7);
        obj.mult_all(2);
        assert_eq!(10, obj.get_index(0));
        obj.add_all(3);
        obj.append(10);
        obj.mult_all(2);
        assert_eq!(26, obj.get_index(0));
        assert_eq!(34, obj.get_index(1));
        assert_eq!(20, obj.get_index(2));
    }

    #[test]
    fn test_1622_solution_1() {
        let mut obj = Fancy::new();
        obj.add_all(1);
        assert_eq!(-1, obj.get_index(0));
    }

    #[test]
    fn test_1622_solution_2() {
        let mut obj = Fancy::new();
        obj.append(12);
        obj.append(8);
        assert_eq!(8, obj.get_index(1));
        obj.append(12);
        assert_eq!(12, obj.get_index(0));
        obj.add_all(12);
        obj.append(8);
        assert_eq!(24, obj.get_index(2));
        assert_eq!(24, obj.get_index(2));
        obj.append(4);
        obj.append(13);
        assert_eq!(4, obj.get_index(4));
        obj.append(12);
        assert_eq!(12, obj.get_index(6));
        obj.append(11);
        assert_eq!(20, obj.get_index(1));
        obj.append(10);
        assert_eq!(24, obj.get_index(2));
        obj.mult_all(3);
        obj.add_all(1);
        assert_eq!(37, obj.get_index(6));
        obj.append(14);
        obj.add_all(5);
        assert_eq!(42, obj.get_index(6));
        obj.mult_all(12);
        assert_eq!(360, obj.get_index(3));
        obj.mult_all(12);
        obj.add_all(15);
        obj.add_all(6);
        obj.append(7);
        obj.mult_all(8);
        obj.append(13);
        obj.append(15);
        obj.append(15);
        obj.mult_all(10);
        assert_eq!(220560, obj.get_index(9));
        obj.mult_all(12);
        obj.mult_all(12);
        obj.mult_all(9);
        assert_eq!(285845760, obj.get_index(9));
        obj.add_all(9);
        obj.append(9);
        obj.mult_all(4);
        obj.add_all(8);
        obj.add_all(11);
        obj.mult_all(15);
        obj.add_all(9);
        obj.add_all(1);
        obj.append(4);
        obj.append(10);
        assert_eq!(150746316, obj.get_index(9));
    }
}
