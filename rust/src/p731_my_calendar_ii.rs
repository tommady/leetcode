// You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a triple booking. A triple booking happens when three events have some non-empty intersection (i.e., some moment is common to all the three events.). The event can be represented as a pair of integers start and end that represents a booking on the half-open interval [start, end), the range of real numbers x such that start = x end. Implement the MyCalendarTwo class: MyCalendarTwo() Initializes the calendar object. boolean book(int start, int end) Returns true if the event can be added to the calendar successfully without causing a triple booking. Otherwise, return false and do not add the event to the calendar. Example 1: Input [ MyCalendarTwo , book , book , book , book , book , book ] [[], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]] Output [null, true, true, true, false, true, true] Explanation MyCalendarTwo myCalendarTwo = new MyCalendarTwo(); myCalendarTwo.book(10, 20); // return True, The event can be booked. myCalendarTwo.book(50, 60); // return True, The event can be booked. myCalendarTwo.book(10, 40); // return True, The event can be double booked. myCalendarTwo.book(5, 15); // return False, The event ca not be booked, because it would result in a triple booking. myCalendarTwo.book(5, 10); // return True, The event can be booked, as it does not use time 10 which is already double booked. myCalendarTwo.book(25, 55); // return True, The event can be booked, as the time in [25, 40) will be double booked with the third event, the time [40, 50) will be single booked, and the time [50, 55) will be double booked with the second event. Constraints: 0 = start end = 109 At most 1000 calls will be made to book.
//
//
// ["MyCalendarTwo","book","book","book","book","book","book"]
// [[],[10,20],[50,60],[10,40],[5,15],[5,10],[25,55]]
#![allow(dead_code)]

struct MyCalendarTwo {
    calendar_index: usize,
    calendar: [(i32, i32); 1001],
    overlaps_index: usize,
    overlaps: [(i32, i32); 1001],
}

/**
 `&self` means the method takes an immutable reference.
If you need a mutable reference, change it to `&mut self` instead.
*/
impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            calendar_index: 0,
            calendar: [(0, 0); 1001],
            overlaps_index: 0,
            overlaps: [(0, 0); 1001],
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        fn max(a: i32, b: i32) -> i32 {
            if a > b {
                return a;
            }
            b
        }

        fn min(a: i32, b: i32) -> i32 {
            if a < b {
                return a;
            }
            b
        }

        for i in 0..self.overlaps_index + 1 {
            let (s, e) = self.overlaps[i];
            if start < e && end > s {
                return false;
            }
        }

        for i in 0..self.calendar_index + 1 {
            let (s, e) = self.calendar[i];
            if start < e && end > s {
                self.overlaps[self.overlaps_index] = (max(start, s), min(end, e));
                self.overlaps_index += 1;
            }
        }
        self.calendar[self.calendar_index] = (start, end);
        self.calendar_index += 1;

        true
    }
}

/**
Your MyCalendarTwo object will be instantiated and called as such:
let obj = MyCalendarTwo::new();
let ret_1: bool = obj.book(start, end);
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_731_solution() {
        let mut obj = MyCalendarTwo::new();
        assert_eq!(true, obj.book(10, 20));
        assert_eq!(true, obj.book(50, 60));
        assert_eq!(true, obj.book(10, 40));
        assert_eq!(false, obj.book(5, 15));
        assert_eq!(true, obj.book(5, 10));
        assert_eq!(true, obj.book(25, 55));
    }
}
