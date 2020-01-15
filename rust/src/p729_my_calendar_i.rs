// Implement a MyCalendar class to store your events. A new event can be added if adding the event will not cause a double booking.
//
// Your class will have the method, book(int start, int end). Formally, this represents a booking on the half open interval [start, end), the range of real numbers x such that start <= x < end.
//
// A double booking happens when two events have some non-empty intersection (ie., there is some time that is common to both events.)
//
// For each call to the method MyCalendar.book, return true if the event can be added to the calendar successfully without causing a double booking. Otherwise, return false and do not add the event to the calendar.
//
// Your class will be called like this: MyCalendar cal = new MyCalendar(); MyCalendar.book(start, end)
// Example 1:
//
// MyCalendar();
// MyCalendar.book(10, 20); // returns true
// MyCalendar.book(15, 25); // returns false
// MyCalendar.book(20, 30); // returns true
// Explanation:
// The first event can be booked.  The second can't because time 15 is already booked by another event.
// The third event can be booked, as the first event takes every time less than 20, but not including 20.
//
//
// Note:
//
// The number of calls to MyCalendar.book per test case will be at most 1000.
// In calls to MyCalendar.book(start, end), start and end are integers in the range [0, 10^9].

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::ops::Bound::Included;

struct MyCalendar {
    set: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            set: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.set.range((Included(&start), Included(&end))).count() >= 2 {
            return false;
        }

        for n in start..=end {
            self.set.insert(n);
        }
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_933_solution1() {
        let mut mc = MyCalendar::new();
        assert_eq!(true, mc.book(10, 20));
        assert_eq!(false, mc.book(15, 25));
        assert_eq!(true, mc.book(20, 30));
    }
    #[test]
    fn test_933_solution2() {
        let mut mc = MyCalendar::new();
        assert_eq!(true, mc.book(20, 29));
        assert_eq!(false, mc.book(13, 22));
        assert_eq!(true, mc.book(44, 50));
        assert_eq!(true, mc.book(1, 7));
        assert_eq!(false, mc.book(2, 10));
        assert_eq!(true, mc.book(14, 20));
        assert_eq!(false, mc.book(19, 25));
        assert_eq!(true, mc.book(36, 42));
        assert_eq!(false, mc.book(45, 50));
        assert_eq!(false, mc.book(47, 50));
    }
}
