use core::*;
use std::fmt;

pub struct EqualTo<T> {
    expected: T,
}

impl<T: fmt::Debug> fmt::Display for EqualTo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl<T: PartialEq + fmt::Debug> Matcher<T> for EqualTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        if self.expected.eq(&actual) {
            success()
        } else {
            Err(format!("was {:?}", actual))
        }
    }
}


pub fn equal_to<T: PartialEq + fmt::Debug>(expected: T) -> EqualTo<T> {
    EqualTo { expected: expected }
}
